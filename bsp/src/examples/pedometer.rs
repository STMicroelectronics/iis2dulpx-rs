use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, mut delay: D, mut int_pin: I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
{
    use iis2dulpx::*;
    use iis2dulpx::prelude::*;

    info!("Configuring the sensor");
    let mut sensor = Iis2dulpx::from_bus(bus, delay.clone());

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
    // Enables embedded functions
    sensor.embedded_state_set(1).await.unwrap();

    delay.delay_ms(10).await;

    sensor
        .embedded_int_cfg_set(EmbeddedIntConfig::Latched)
        .await.unwrap();
    sensor.stpcnt_debounce_set(4).await.unwrap();

    let stpcnt_mode = StpcntMode {
        false_step_rej: 0,
        step_counter_enable: 1,
        step_counter_in_fifo: 0,
    };
    sensor.stpcnt_mode_set(&stpcnt_mode).await.unwrap();
    sensor.stpcnt_rst_step_set().await.unwrap();

    let int1_route = EmbPinIntRoute {
        step_det: 1,
        ..Default::default()
    };
    sensor.emb_pin_int1_route_set(&int1_route).await.unwrap();

    let int_config = IntConfig {
        int_cfg: IntCfg::Level,
        ..Default::default()
    };
    sensor.int_config_set(&int_config).await.unwrap();

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

        let status = sensor.embedded_status_get().await.unwrap();
        if status.is_step_det == 1 {
            let steps = sensor.stpcnt_steps_get().await.unwrap();
            writeln!(tx, "Steps: {}", steps).unwrap();
        }
    }}
