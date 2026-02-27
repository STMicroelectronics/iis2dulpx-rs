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

    let wakeup_cfg = WakeupConfig {
        wake_dur: WakeDur::_0Odr,
        sleep_dur: 0,
        wake_ths_weight: 1,
        wake_ths: 2,
        wake_enable: WakeEnable::SleepOn,
        inact_odr: InactOdr::NoChange,
    };
    sensor.wakeup_config_set(wakeup_cfg).await.unwrap();

    // Configure interrupt pins
    let int1_route = PinInt1Route {
        wake_up: 1,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int1_route).await.unwrap();

    let int_mode = IntConfig {
        int_cfg: IntCfg::Latched,
        ..Default::default()
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
        if status_all.wake_up == 1 {
            writeln!(tx, "WAKEUP event detected").unwrap();
        }
    }
}
