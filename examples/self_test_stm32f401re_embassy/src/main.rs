#![no_std]
#![no_main]

use core::fmt::Write;
use core::ops::RangeInclusive;

use embassy_executor::Spawner;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{self, BufferedInterruptHandler, DataBits, Parity, UartTx};
use embassy_stm32::{bind_interrupts, peripherals, peripherals::USART2};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use heapless::String;
use iis2dulpx_rs::prelude::*;
use iis2dulpx_rs::{I2CAddress, Iis2dulpx};

use st_mems_bus::BusOperation;

use {defmt_rtt as _, panic_probe as _};

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!")
}

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[repr(u8)]
enum SelfTestSign {
    Positive = 0,
    Negative = 1,
}

const ST_RANGE_DEV_X: RangeInclusive<u32> = 50..=700;
const ST_RANGE_DEV_Y: RangeInclusive<u32> = 50..=700;
const ST_RANGE_DEV_Z: RangeInclusive<u32> = 200..=1200;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut usart_cfg = usart::Config::default();
    usart_cfg.baudrate = 115200;
    usart_cfg.data_bits = DataBits::DataBits8;
    usart_cfg.parity = Parity::ParityNone;

    let mut tx = UartTx::new(p.USART2, p.PA2, p.DMA1_CH6, usart_cfg).unwrap();

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH7,
        p.DMA1_CH5,
        khz(100),
        Default::default(),
    );

    let mut delay = Delay;
    let mut msg = String::<64>::new();

    delay.delay_ms(10);

    let mut sensor = Iis2dulpx::new_i2c(i2c, I2CAddress::I2cAddH, delay);

    match sensor.device_id_get() {
        Ok(value) => {
            if value != iis2dulpx_rs::ID {
                panic!("Invalid sensor ID")
            }
        }
        Err(e) => {
            writeln!(&mut msg, "An error occured while reading sensor ID: {e:?}").unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        }
    }
    sensor.tim.delay_ms(25);

    // Restore default configuration
    sensor.init_set(Init::Reset).unwrap();
    loop {
        let status = sensor.status_get().unwrap();
        if status.sw_reset == 0 {
            break;
        }
    }

    // Accelerometer self test
    for test in [SelfTestSign::Positive, SelfTestSign::Negative].iter() {
        // 1. Set the device in soft power-down e wait 10ms
        power_down_and_wait(&mut sensor).unwrap();

        // 2. Set the FIFO_EN bit in the CTRL4 (13h) register to 1.
        // 3. Set the XL_ONLY_FIFO bit in the FIFO_WTM (16h) register to 1.
        // 5. Set the FIFO_CTRL (15h) register to 00h to empty the FIFO.
        let mut fifo_mode = sensor.fifo_mode_get().unwrap();
        fifo_mode.operation = FifoOperation::BypassMode;
        fifo_mode.xl_only = 1;
        fifo_mode.store = Store::Fifo1x;
        sensor.fifo_mode_set(&fifo_mode).unwrap();

        // 4. Set the ST_SIGN_X and ST_SIGN_Y bits in the CTRL3 (12h) register to 1
        //   and the ST_SIGN_Z bit in the WAKE_UP_DUR (1Dh) register to 0
        //   (i.e. 001 for POSITIVE. Instead, for NEGATIVE is 010).
        match test {
            SelfTestSign::Positive => sensor.self_test_sign_set(XlSelfTest::Positive).unwrap(),
            SelfTestSign::Negative => sensor.self_test_sign_set(XlSelfTest::Negative).unwrap(),
        }

        // 6. Set ST[1:0] to "10"
        sensor.self_test_start(2).unwrap();

        // 7. Set ODR = 200 hz, BW = ODR/2, FS = +/- 8 g from the CTRL5 (14h) register and wait 50ms.
        let md = Md {
            odr: Odr::_200hzLp,
            fs: Fs::_8g,
            bw: Bw::OdrDiv2,
        };
        sensor.mode_set(&md).unwrap();
        sensor.tim.delay_ms(50);

        // 8. Set tje FIFO_CTRL (15h) register to 01h to start filling the FIFO.
        let mut fifo_mode = sensor.fifo_mode_get().unwrap();
        fifo_mode.operation = FifoOperation::FifoMode;
        sensor.fifo_mode_set(&fifo_mode).unwrap();

        // 9. Read the first 5 samples from FIFO, compute the average for each axis and save the result.
        (0..5).for_each(|_| _ = sensor.fifo_data_level_get().unwrap());
        let out1 = st_avg_5_samples(&mut sensor, &md, &fifo_mode).unwrap();

        // 10. Set the device in power down mode and wait 10ms
        power_down_and_wait(&mut sensor).unwrap();

        // 11. Set the FIFO_CTRL (15h) register to 00 to empty the FIFO.
        let mut fifo_mode = sensor.fifo_mode_get().unwrap();
        fifo_mode.operation = FifoOperation::BypassMode;
        sensor.fifo_mode_set(&fifo_mode).unwrap();

        // 12. Set ST[1:0] to "01"
        sensor.self_test_start(1).unwrap();

        // 13. Set ODR = 200 hz, BW = ODR/2, FS = +/- 8 g from the CTRL5 (14h) register and wait 50ms.
        sensor.mode_set(&md).unwrap();
        sensor.tim.delay_ms(50);

        // 14. Set the FIFO_CTRL (15h) register to 01h to start filling the FIFO and wait 25ms.
        let mut fifo_mode = sensor.fifo_mode_get().unwrap();
        fifo_mode.operation = FifoOperation::FifoMode;
        sensor.fifo_mode_set(&fifo_mode).unwrap();
        sensor.tim.delay_ms(25);

        // 15. Read the first 5 samples from FIFO, compute the average for each axis, and save the resutl in OUT2.
        (0..5).for_each(|_| _ = sensor.fifo_data_level_get().unwrap());
        let out2 = st_avg_5_samples(&mut sensor, &md, &fifo_mode).unwrap();

        // 16. Set the device in power down mode and wait 10ms
        power_down_and_wait(&mut sensor).unwrap();

        // 17. Set the ST[1:0] bits in the SELF_TEST (32h) register to 00.
        sensor.self_test_stop().unwrap();

        // 18. Self-test deviation is 2 * |OUT2 - OUT1|. Compute the value for each axis and verify that it falls
        // within the range provided in the datasheet
        let mut st_dev = FifoData::default();
        (0..3).for_each(|i| st_dev.xl[0].mg[i] = 2. * (out2.xl[0].mg[i] - out1.xl[0].mg[i]).abs());

        // 19. Set device in power down mode
        power_down_and_wait(&mut sensor).unwrap();

        // Chech if st_dev falls into given ranges
        let passed = ST_RANGE_DEV_X.contains(&(st_dev.xl[0].mg[0] as u32))
            && ST_RANGE_DEV_Y.contains(&(st_dev.xl[0].mg[1] as u32))
            && ST_RANGE_DEV_Z.contains(&(st_dev.xl[0].mg[2] as u32));

        if passed {
            writeln!(
                &mut msg,
                "{} Self Test - PASS",
                match test {
                    SelfTestSign::Positive => "POS",
                    SelfTestSign::Negative => "NEG",
                }
            )
            .unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        } else {
            writeln!(
                &mut msg,
                "{} Self Test - FAIL!!!!",
                match test {
                    SelfTestSign::Positive => "POS",
                    SelfTestSign::Negative => "NEG",
                }
            )
            .unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        }
    }

    loop {}
}

fn power_down_and_wait<B, T>(
    sensor: &mut Iis2dulpx<B, T>,
) -> Result<(), iis2dulpx_rs::Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let md = Md {
        fs: Fs::_8g,
        odr: Odr::Off,
        ..Default::default()
    };
    sensor.mode_set(&md)?;
    sensor.tim.delay_ms(10);
    Ok(())
}

fn st_avg_5_samples<B, T>(
    sensor: &mut Iis2dulpx<B, T>,
    md: &Md,
    fifo_md: &FifoMode,
) -> Result<FifoData, iis2dulpx_rs::Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let mut fdata = FifoData::default();

    (0..5).for_each(|_| {
        let tmp = sensor.fifo_data_get(md, fifo_md).unwrap();

        fdata.xl[0].mg[0] += tmp.xl[0].mg[0];
        fdata.xl[0].mg[1] += tmp.xl[0].mg[1];
        fdata.xl[0].mg[2] += tmp.xl[0].mg[2];
    });

    fdata.xl[0].mg[0] /= 5.;
    fdata.xl[0].mg[1] /= 5.;
    fdata.xl[0].mg[2] /= 5.;

    Ok(fdata)
}
