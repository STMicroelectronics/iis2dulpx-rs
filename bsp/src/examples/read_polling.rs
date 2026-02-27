use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L>(bus: B, mut tx: L, mut delay: D, _irq: ()) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write
{
    use iis2dulpx::prelude::*;
    use iis2dulpx::*;

    info!("Configuring the sensor");
    let mut sensor = Iis2dulpx::from_bus(bus, delay.clone());

    // Exit deep power down
    // sensor.exit_deep_power_down().await.unwrap(); // Only SPI

    // boot time
    sensor.tim.delay_ms(5).await;

    // Check device ID
    let id = sensor.device_id_get().await.unwrap();
    info!("Device ID: {:x}", id);
    if id != ID {
        info!("Unexpected device ID: {:x}", id);
        writeln!(tx, "Unexpected device ID: {:x}", id).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.sw_reset().await.unwrap();

    // Set BDU and IF_INC recommended for driver usage
    sensor.init_set().await.unwrap();

    // Set Output Data Rate
    let md = Md {
        fs: Fs::_4g,
        bw: Bw::OdrDiv4,
        odr: Odr::_25hzLp,
    };
    sensor.mode_set(&md).await.unwrap();

    // Read samples in polling mode (no int)
    loop {
        // Read output only if new values are available
        if sensor.status_get().await.unwrap().drdy != 0 {
            // Read acceleration data
            match sensor.xl_data_get(&md).await {
                Ok(data_xl) => {
                    writeln!(
                        tx,
                        "Acceleration [mg]: {:.2}\t{:.2}\t{:.2}",
                        data_xl.mg[0], data_xl.mg[1], data_xl.mg[2]
                    )
                    .unwrap();
                }
                Err(e) => writeln!(tx, "Error in reading acceleration data: {:?}", e).unwrap(),
            }

            // Read temperature data
            match sensor.outt_data_get().await {
                Ok(data_temp) => {
                    writeln!(tx, "Temp [degC]: {:.2}", data_temp.heat.deg_c).unwrap();
                }
                Err(e) => writeln!(tx, "Error in reading temperature data: {:?}", e).unwrap(),
            }
        }
        delay.delay_ms(1000_u32).await;
    }

}
