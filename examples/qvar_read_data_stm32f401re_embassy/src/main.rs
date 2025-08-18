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
use iis2dulpx_rs::{I2CAddress, Iis2dulpx, PROPERTY_ENABLE};

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

    let mut int_pin = ExtiInput::new(p.PA10, p.EXTI10, Pull::None);

    let mut delay = Delay;
    let mut msg = String::<64>::new();

    delay.delay_ms(10);

    let mut sensor = Iis2dulpx::new_i2c(i2c, I2CAddress::I2cAddL, delay);

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
    sensor.init_set(Init::SensorOnlyOn).unwrap();

    // Configure interrupt pins
    let int1_route = PinIntRoute {
        drdy: PROPERTY_ENABLE,
        int_on_res: PROPERTY_ENABLE,
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

    // Enable AH/QVAR function
    let qvar_mode = AhQvarMode {
        ah_qvar_en: PROPERTY_ENABLE,
        ah_qvar_zin: AhQvarZin::_520mohm,
        ah_qvar_gain: AhQvarGain::_05,
        ..Default::default()
    };
    sensor.ah_qvar_mode_set(&qvar_mode).unwrap();

    // Read qvar samples at drdy
    loop {
        int_pin.wait_for_rising_edge().await;
        let data_qvar = sensor.ah_qvar_data_get().unwrap();
        let data_xl = sensor.xl_data_get(&md).unwrap();

        writeln!(
            &mut msg,
            "Acceleration [mg]: {:4.2}  {:4.2}  {:4.2} - QVAR [LSB]: {}",
            data_xl.mg[0], data_xl.mg[1], data_xl.mg[2], data_qvar.raw
        )
        .unwrap();
        tx.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
