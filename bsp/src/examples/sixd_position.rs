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

    delay.delay_ms(10).await;

    let sixd_config = SixdConfig {
        mode: Mode::_6d,
        threshold: Threshold::_60deg,
    };
    sensor.sixd_config_set(sixd_config).await.unwrap();

    // Configure interrupt pins
    let int1_route = PinInt1Route {
        six_d: 1,
        ..Default::default()
    };
    sensor.pin_int1_route_set(&int1_route).await.unwrap();

    let int_mode = IntConfig {
        int_cfg: IntCfg::Level,
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

        let status = sensor.all_sources_get().await.unwrap();
        let sixd_event = (status.six_d << 6)
            | (status.six_d_zh << 5)
            | (status.six_d_zl << 4)
            | (status.six_d_yh << 3)
            | (status.six_d_yl << 2)
            | (status.six_d_xh << 1)
            | status.six_d_xl;
        if sixd_event & 0x40 != 0 {
            if sixd_event & 0x8 != 0 {
                writeln!(tx, "Y-axis UP").unwrap();
            } else if sixd_event & 0x01 != 0 {
                writeln!(tx, "X-axis DOWN").unwrap();
            } else if sixd_event & 0x02 != 0 {
                writeln!(tx, "X-axis UP").unwrap();
            } else if sixd_event & 0x04 != 0 {
                writeln!(tx, "Y-axis DOWN").unwrap();
            } else if sixd_event & 0x20 != 0 {
                writeln!(tx, "Z-axis UP").unwrap();
            } else if sixd_event & 0x10 != 0 {
                writeln!(tx, "Z-axis DOWN").unwrap();
            }
        }
    }}
