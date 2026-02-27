use defmt::info;
use maybe_async::maybe_async;
use crate::*;

use crate::config::mlc_config::VIBRATION;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

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

    for ucf_entry in VIBRATION {
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
        let status = sensor.mlc_status_get().await.unwrap();

        if status.is_mlc1() == 1 {
            let mlcs_buff = sensor.mlc_out_get().await.unwrap();

            let msg = match mlcs_buff[0] {
                0 => "No vibration",
                1 => "Low vibration",
                2 => "High vibration",
                _ => "",
            };

            writeln!(tx, "{}", msg).unwrap();
        }
    }
}
