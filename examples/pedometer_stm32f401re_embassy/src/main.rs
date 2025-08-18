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

    let mut int_pin = ExtiInput::new(p.PA4, p.EXTI4, Pull::None);

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
        ..Default::default()
    };

    sensor.stpcnt_mode_set(&stpcnt_mode).unwrap();

    sensor.stpcnt_rst_step_set().unwrap();

    // Configure interrupt pins
    let int1_route = EmbPinIntRoute {
        step_det: PROPERTY_ENABLE,
        ..Default::default()
    };
    sensor.emb_pin_int1_route_set(&int1_route).unwrap();

    let int_mode = IntConfig {
        int_cfg: IntCfg::Level,
        ..Default::default()
    };
    sensor.int_config_set(&int_mode).unwrap();

    // Set Output Data Rate
    let md = Md {
        odr: Odr::_25hzLp,
        fs: Fs::_4g,
        bw: Bw::OdrDiv4,
    };
    sensor.mode_set(&md).unwrap();

    // Wait forever (xl samples read with drdy irq)
    loop {
        int_pin.wait_for_rising_edge().await;

        let status = sensor.embedded_status_get().unwrap();

        if status.is_step_det == 1 {
            let steps = sensor.stpcnt_steps_get().unwrap();

            writeln!(&mut msg, "Steps: {}", steps).unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        }
    }
}
