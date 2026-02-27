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

    let int_mode = IntConfig {
        int_cfg: IntCfg::Latched,
        sleep_status_on_int: 0,
        dis_rst_lir_all_int: 0,
    };
    sensor.int_config_set(&int_mode).await.unwrap();

    let tap_cfg = TapConfig {
        axis: Axis::TapOnZ,
        pre_still_ths: (125.0 / 62.5) as u8,
        post_still_ths: (500.0 / 62.5) as u8,
        post_still_time: (32 / 4) as u8,
        peak_ths: (500.0 / 62.5) as u8,
        pre_still_start: 0,
        pre_still_n: 10,
        inverted_peak_time: 4,
        shock_wait_time: (6.0 / 2.0) as u8,
        rebound: 0,
        latency: (128 / 32) as u8,
        single_tap_on: 1,
        double_tap_on: 1,
        triple_tap_on: 1,
        wait_end_latency: 1,
    };

    sensor.tap_config_set(tap_cfg).await.unwrap();

    // Configure interrupt pins
    let int1_route = PinInt1Route {
        tap: 1,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int1_route).await.unwrap();

    // Set Output Data Rate
    let md = Md {
        fs: Fs::_8g,
        bw: Bw::OdrDiv2,
        odr: Odr::_400hzLp,
    };
    sensor.mode_set(&md).await.unwrap();

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status = sensor.all_sources_get().await.unwrap();

        if status.single_tap == 1 {
            writeln!(tx, "single TAP detected").unwrap();
        }
        if status.double_tap == 1 {
            writeln!(tx, "double TAP detected").unwrap();
        }
        if status.triple_tap == 1 {
            writeln!(tx, "triple TAP detected").unwrap();
        }
    }
}
