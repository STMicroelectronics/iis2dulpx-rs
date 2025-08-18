#![no_main]
#![no_std]

use core::{cell::RefCell, fmt::Write};

use cortex_m::interrupt::{free, Mutex};
use iis2dulpx_rs::prelude::*;
use iis2dulpx_rs::{I2CAddress, Iis2dulpx, PROPERTY_ENABLE};
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

// static FIFO_INT: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
static INT_PIN: Mutex<RefCell<Option<gpio::PA4<Input>>>> = Mutex::new(RefCell::new(None));
static MEMS_EVENT: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

const NUM_FIFO_ENTRY: u8 = 33; // 32 samples + timestamp

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

    // Set bdu and if_inc recommended for driver usage
    sensor.init_set(Init::SensorOnlyOn).unwrap();

    // Set FIFO watermark to 32 samples
    let fifo_mode = FifoMode {
        operation: FifoOperation::StreamMode,
        store: Store::Fifo1x,
        xl_only: 0,
        watermark: NUM_FIFO_ENTRY,
        fifo_event: FifoEvent::Wtm,
        batch: Batch {
            dec_ts: DecTs::_32,
            bdr_xl: BdrXl::Odr,
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

    // Wait forever (FIFO samples read with irq)
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
        let wm_flag = sensor.fifo_wtm_flag_get().unwrap();
        if wm_flag > 0 {
            // Read number of samples in FIFO
            let num = sensor.fifo_data_level_get().unwrap();
            writeln!(tx, "-- {num} in FIFO").unwrap();

            (0..num as usize).rev().for_each(|i| {
                    let fdata = sensor.fifo_data_get(&md, &fifo_mode).unwrap();

                    match FifoSensorTag::try_from(fdata.tag).unwrap_or_default() {
                        FifoSensorTag::XlOnly2xTag => {
                            writeln!(
                                tx,
                                "{:02}_0: Acceleration [0][mg]:\t{:4.2}\t{:4.2}\t{:4.2}",
                                NUM_FIFO_ENTRY - i as u8,
                                fdata.xl[0].mg[0],
                                fdata.xl[0].mg[1],
                                fdata.xl[0].mg[2]
                            )
                            .unwrap();
                            writeln!(
                                tx,
                                "{:02}_1: Acceleration [1][mg]:\t{:4.2}\t{:4.2}\t{:4.2}",
                                NUM_FIFO_ENTRY - i as u8,
                                fdata.xl[1].mg[0],
                                fdata.xl[1].mg[1],
                                fdata.xl[1].mg[2]
                            )
                            .unwrap();
                        }
                        FifoSensorTag::XlTempTag => {
                            if fifo_mode.xl_only == 0 {
                             writeln!(
                                tx,
                                "{:02}: Acceleration [mg]:\t{:4.2}\t{:4.2}\t{:4.2}\r\nTemp [degC]:\t{:3.2}",
                                NUM_FIFO_ENTRY - i as u8,
                                fdata.xl[0].mg[0],
                                fdata.xl[0].mg[1],
                                fdata.xl[0].mg[2],
                                fdata.heat.deg_c
                            )
                            .unwrap();
                            } else {
                                writeln!(
                                tx,
                                "{:02}: Acceleration [mg]:\t{:4.2}\t{:4.2}\t{:4.2}",
                                NUM_FIFO_ENTRY - i as u8,
                                fdata.xl[0].mg[0],
                                fdata.xl[0].mg[1],
                                fdata.xl[0].mg[2]
                            )
                            .unwrap();
                            }
                        },
                        FifoSensorTag::TimestampTag => {
                            let ts = fdata.cfg_chg.timestamp / 100;
                            writeln!(tx, "Timestamp:\t{ts}").unwrap();
                        },
                        _ => writeln!(tx, "unkown TAG ({:02x})", fdata.tag).unwrap()
                    }
                });
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
