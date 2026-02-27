use defmt::info;
use maybe_async::maybe_async;
use crate::*;

static NUM_FIFO_ENTRY: u8 = 8;

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
        step_counter_in_fifo: 1,
    };
    sensor.stpcnt_mode_set(&stpcnt_mode).await.unwrap();
    sensor.stpcnt_rst_step_set().await.unwrap();

    // Set FIFO mode
    let fifo_mode = FifoMode {
        operation: FifoOperation::StreamMode,
        store: Store::Fifo2x,
        xl_only: 0,
        cfg_change_in_fifo: 0,
    };
    let batch = Batch {
        dec_ts: DecTs::_1,
        bdr_xl: BdrXl::OdrOff,
    };
    sensor.fifo_mode_set(&fifo_mode).await.unwrap();
    sensor.fifo_watermark_set(NUM_FIFO_ENTRY).await.unwrap();
    sensor.fifo_batch_set(&batch).await.unwrap();
    sensor.fifo_stop_on_wtm_set(FifoEvent::Wtm).await.unwrap();
    sensor.timestamp_set(1).await.unwrap();

    // Configure interrupt pins
    let int1_route = PinInt1Route {
        fifo_th: 1,
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

        let wmflag = sensor.fifo_wtm_flag_get().await.unwrap();

        if wmflag > 0 {
            let num = sensor.fifo_data_level_get().await.unwrap();

            writeln!(tx, "-- {} in FIFO", num).unwrap();

            let fdata = sensor.fifo_data_get(&md, &fifo_mode).await.unwrap();

            if let FifoSensorTag::StepCounterTag =
                FifoSensorTag::try_from(fdata.tag).unwrap_or_default()
            {
                let ts = fdata.pedo.timestamp as f32 / 100f32;
                let steps = fdata.pedo.steps;

                writeln!(tx, "Steps: {:03} ({} ms)", steps, ts).unwrap();
            }
        }
    }
}
