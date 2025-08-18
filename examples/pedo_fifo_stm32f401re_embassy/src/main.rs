#![no_std]
#![no_main]

use core::fmt::Write;

use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Pull;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{self, BufferedInterruptHandler, DataBits, Parity, UartTx};
use embassy_stm32::{bind_interrupts, peripherals, peripherals::USART2};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use heapless::String;
use iis2dulpx_rs::prelude::*;
use iis2dulpx_rs::{I2CAddress, Iis2dulpx, PROPERTY_DISABLE, PROPERTY_ENABLE};

use {defmt_rtt as _, panic_probe as _};

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!")
}

const NUM_FIFO_ENTRY: u8 = 8;

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

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

    let mut int_pin = ExtiInput::new(p.PA4, p.EXTI4, Pull::None);

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

    // Set bdu and if_inc recommended for driver usage
    sensor.init_set(Init::SensorEmbFuncOn).unwrap();
    sensor.tim.delay_ms(10);

    sensor
        .embedded_int_cfg_set(EmbeddedIntConfig::Latched)
        .unwrap();

    sensor.stpcnt_debounce_set(4).unwrap();

    let stpcnt_mode = StpcntMode {
        step_counter_enable: PROPERTY_ENABLE,
        false_step_rej: PROPERTY_DISABLE,
        step_counter_in_fifo: PROPERTY_ENABLE,
        ..Default::default()
    };
    sensor.stpcnt_mode_set(&stpcnt_mode).unwrap();
    sensor.stpcnt_rst_step_set().unwrap();

    // Set FIFO mode
    let fifo_mode = FifoMode {
        operation: FifoOperation::StreamMode,
        store: Store::Fifo2x,
        watermark: NUM_FIFO_ENTRY,
        fifo_event: FifoEvent::Wtm,
        batch: Batch {
            dec_ts: DecTs::_1,
            bdr_xl: BdrXl::OdrOff,
        },
        ..Default::default()
    };
    sensor.fifo_mode_set(&fifo_mode).unwrap();

    sensor.timestamp_set(PROPERTY_ENABLE).unwrap();

    // Configure interrupt pins
    let int1_route = PinIntRoute {
        fifo_th: PROPERTY_ENABLE,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int1_route).unwrap();

    // Set Output Data Rate
    let md = Md {
        odr: Odr::_25hzLp,
        fs: Fs::_4g,
        bw: Bw::OdrDiv4,
    };
    sensor.mode_set(&md).unwrap();

    let _ = sensor.fifo_data_get(&md, &fifo_mode).unwrap();

    // Wait forever (FIFO samples read with irq)
    loop {
        int_pin.wait_for_rising_edge().await;

        let wm_flag = sensor.fifo_wtm_flag_get().unwrap();
        if wm_flag > 0 {
            // Read number of samples in FIFO
            let num = sensor.fifo_data_level_get().unwrap();
            writeln!(&mut msg, "-- {num} in FIFO").unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();

            (0..num as usize).rev().for_each(|_| {
                let fdata = sensor.fifo_data_get(&md, &fifo_mode).unwrap();

                match FifoSensorTag::try_from(fdata.tag).unwrap_or_default() {
                    FifoSensorTag::StepCounterTag => {
                        let ts = fdata.pedo.timestamp / 100;
                        let steps = fdata.pedo.steps;

                        writeln!(&mut msg, "Steps: {:03} ({} ms)", steps, ts).unwrap();
                        tx.blocking_write(msg.as_bytes()).unwrap();
                        msg.clear();
                    }
                    _ => {
                        writeln!(&mut msg, "unkown TAG ({:02x})", fdata.tag).unwrap();
                        tx.blocking_write(msg.as_bytes()).unwrap();
                        msg.clear();
                    }
                }
            });
        }
    }
}
