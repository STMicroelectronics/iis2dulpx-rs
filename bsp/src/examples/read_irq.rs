use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, delay: D, mut int_pin: I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
{
    use iis2dulpx::*;
    use iis2dulpx::prelude::*;

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

    let int1_route = PinInt1Route {
        drdy: 1,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int1_route).await.unwrap();

    // Set Output Data Rate
    let md = Md {
        fs: Fs::_4g,
        bw: Bw::OdrDiv4,
        odr: Odr::_25hzLp,
    };
    sensor.mode_set(&md).await.unwrap();

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

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
    }

}
