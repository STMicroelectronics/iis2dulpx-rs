#![no_std]
#![no_main]

use core::fmt::{Display, Write};

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
use iis2dulpx_rs::{I2CAddress, Iis2dulpx};

mod fsm_config;

use fsm_config::FOUR_D;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

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
enum FourdEvent {
    PortraitDown,
    PortraitUp,
    LandscapeRight,
    LandscapeLeft,
    Unknown(u8),
}

impl From<u8> for FourdEvent {
    fn from(value: u8) -> Self {
        match value {
            0x10 => Self::PortraitDown,
            0x20 => Self::PortraitUp,
            0x40 => Self::LandscapeRight,
            0x80 => Self::LandscapeLeft,
            other => Self::Unknown(other),
        }
    }
}

impl Display for FourdEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FourdEvent::PortraitDown => write!(f, "Y-axis pointing down"),
            FourdEvent::PortraitUp => write!(f, "Y-axis pointing up"),
            FourdEvent::LandscapeRight => write!(f, "X-axis pointing down"),
            FourdEvent::LandscapeLeft => write!(f, "X-axis pointing up"),
            FourdEvent::Unknown(v) => write!(f, "Unkown event: {v}"),
        }
    }
}

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

    // Start Matchine Learning Core configuration
    for ufc_line in FOUR_D {
        match ufc_line.op {
            MemsUcfOp::Delay => sensor.tim.delay_ms(ufc_line.data.into()),
            MemsUcfOp::Write => sensor
                .write_to_register(ufc_line.address, &[ufc_line.data])
                .unwrap(),
            _ => {}
        }
    }

    loop {
        int_pin.wait_for_rising_edge().await;

        let status = sensor.fsm_status_get().unwrap();
        if status.is_fsm1() == 1 {
            let catched: FourdEvent = sensor.fsm_out_get().unwrap()[0].into();
            writeln!(&mut msg, "{catched}").unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        }
    }
}
