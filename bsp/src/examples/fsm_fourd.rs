use defmt::info;
use maybe_async::maybe_async;
use crate::*;
use core::write;

use crate::config::fsm_config::FOURD;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

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

impl core::fmt::Display for FourdEvent {
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

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, delay: D, mut int_pin: I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
{
    use iis2dulpx::*;

    info!("Configuring the sensor");
    let mut sensor = Iis2dulpx::from_bus(bus, delay);

    // sensor.exit_deep_power_down().await.unwrap(); // Only SPI

    // Check device ID
    let whoami = sensor.device_id_get().await.unwrap();
    info!("Device ID: {:x}", whoami);
    if whoami != ID {
        writeln!(tx, "Device ID mismatch: {:#02x}", whoami).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.sw_reset().await.unwrap();

    // Set BDU and IF_INC recommended for driver usage
    sensor.init_set().await.unwrap();

    for ucf_entry in FOURD {
        match ucf_entry.op {
            MemsUcfOp::Delay => {
                sensor.tim.delay_ms(ucf_entry.data.into()).await;
            }
            MemsUcfOp::Write => {
                sensor
                    .bus
                    .write_to_register(ucf_entry.address as u8, &[ucf_entry.data])
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status = sensor.fsm_status_get().await.unwrap();
        if status.is_fsm1() == 1 {
            let catched: FourdEvent = sensor.fsm_out_get().await.unwrap()[0].into();
            writeln!(tx, "{catched}").unwrap();
        }
    }}
