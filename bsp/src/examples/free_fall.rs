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
    use iis2dulpx::prelude::*;
    use iis2dulpx::*;

    info!("Configuring the sensor");
    let mut sensor = Iis2dulpx::from_bus(bus, delay);

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

    sensor.ff_duration_set(10).await.unwrap();
    sensor.ff_thresholds_set(FfThreshold::_312mg).await.unwrap();

    // Configure interrupt pins
    let int1_route = PinInt1Route {
        free_fall: 1,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int1_route).await.unwrap();

    let int_mode = IntConfig {
        int_cfg: IntCfg::Latched,
        sleep_status_on_int: 0,
        dis_rst_lir_all_int: 0,
    };
    sensor.int_config_set(&int_mode).await.unwrap();

    // Set Output Data Rate
    let md = Md {
        fs: Fs::_2g,
        bw: Bw::OdrDiv2,
        odr: Odr::_200hzLp,
    };
    sensor.mode_set(&md).await.unwrap();

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status_all = sensor.all_sources_get().await.unwrap();
        if status_all.free_fall == 1 {
            writeln!(tx, "Free-Fall detected!").unwrap();
        }
    }

}
