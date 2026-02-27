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

    // Set Output Data Rate
    let md = Md {
        fs: Fs::_4g,
        bw: Bw::OdrDiv4,
        odr: Odr::_25hzLp,
    };
    sensor.mode_set(&md).await.unwrap();

    let qvar_mode = AhQvarMode {
        ah_qvar_en: 1,
        ah_qvar_zin: AhQvarZin::_520mohm,
        ah_qvar_gain: AhQvarGain::_05,
        ..Default::default()
    };
    sensor.ah_qvar_mode_set(&qvar_mode).await.unwrap();

    let int_route = PinInt1Route {
        drdy: 1,
        int_on_res: 1,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int_route).await.unwrap();

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status = sensor.status_get().await.unwrap();

        if status.drdy == 1 {
            let data_qvar = sensor.ah_qvar_data_get().await.unwrap();
            let data_xl = sensor.xl_data_get(&md).await.unwrap();

            writeln!(
                tx,
                "Acceleration [mg]:{:4.2}\t{:4.2}\t{:4.2} - QVAR [LSB]: {}",
                data_xl.mg[0], data_xl.mg[1], data_xl.mg[2], data_qvar.raw
            )
            .unwrap();
        }
    }}
