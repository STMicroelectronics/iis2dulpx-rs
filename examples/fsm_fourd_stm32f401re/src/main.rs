#![no_main]
#![no_std]

use core::{
    cell::RefCell,
    fmt::{Display, Write},
};

use cortex_m::interrupt::{free, Mutex};
use iis2dulpx_rs::prelude::*;
use iis2dulpx_rs::{I2CAddress, Iis2dulpx};
use panic_itm as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    gpio::{self, Edge, Input},
    hal::delay::DelayNs,
    i2c::{DutyCycle, I2c, Mode},
    pac::{self, interrupt},
    prelude::*,
    serial::Config,
};

mod fsm_config;
use fsm_config::FOUR_D;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

static INT_PIN: Mutex<RefCell<Option<gpio::PA4<Input>>>> = Mutex::new(RefCell::new(None));
static MEMS_EVENT: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

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

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let scl = gpiob.pb8;
    let sda = gpiob.pb9;

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        &clocks,
    );

    let tx_pin = gpioa.pa2.into_alternate();
    let mut tx = dp
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();

    let mut syscfg = dp.SYSCFG.constrain();

    let mut int_pin = gpioa.pa4.into_input();
    // Configure pin for interrupts
    int_pin.make_interrupt_source(&mut syscfg);
    int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    int_pin.enable_interrupt(&mut dp.EXTI);

    // Enable interrupts
    let int_pin_num = int_pin.interrupt();
    pac::NVIC::unpend(int_pin_num);
    unsafe {
        pac::NVIC::unmask(int_pin_num);
    };

    free(|cs| INT_PIN.borrow(cs).replace(Some(int_pin)));

    delay.delay_ms(5);

    let mut sensor = Iis2dulpx::new_i2c(i2c, I2CAddress::I2cAddH, delay);

    match sensor.device_id_get() {
        Ok(value) => {
            if value != iis2dulpx_rs::ID {
                panic!("Invalid sensor ID")
            }
        }
        Err(e) => writeln!(tx, "An error occured while reading sensor ID: {e:?}").unwrap(),
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
        // Wait for interrupt
        let mems_event = cortex_m::interrupt::free(|cs| {
            let flag = *MEMS_EVENT.borrow(cs).borrow();
            if flag {
                MEMS_EVENT.borrow(cs).replace(false);
            }
            flag
        });
        if !mems_event {
            continue;
        }
        let status = sensor.fsm_status_get().unwrap();
        if status.is_fsm1() == 1 {
            let catched: FourdEvent = sensor.fsm_out_get().unwrap()[0].into();
            writeln!(tx, "{catched}").unwrap();
        }
    }
}

#[interrupt]
fn EXTI4() {
    cortex_m::interrupt::free(|cs| {
        // Obtain access to Peripheral and Clear Interrupt Pending Flag
        let mut int_pin = INT_PIN.borrow(cs).borrow_mut();
        if int_pin.as_mut().unwrap().check_interrupt() {
            int_pin.as_mut().unwrap().clear_interrupt_pending_bit();
        }
        MEMS_EVENT.borrow(cs).replace(true);
    });
}
