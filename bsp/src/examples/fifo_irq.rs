use defmt::info;
use maybe_async::maybe_async;
use crate::*;

static NUM_FIFO_ENTRY: u8 = 33; // 32 samples + timestamp

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

    // Set FIFO watermark to 32 sample(s)
    let fifo_mode = FifoMode {
        store: Store::Fifo1x,
        xl_only: 0,
        operation: FifoOperation::StreamMode,
        cfg_change_in_fifo: 0,
    };
    let batch = Batch {
        dec_ts: DecTs::_32,
        bdr_xl: BdrXl::Odr,
    };
    sensor.fifo_mode_set(&fifo_mode).await.unwrap();
    sensor.fifo_watermark_set(NUM_FIFO_ENTRY).await.unwrap();
    sensor.fifo_batch_set(&batch).await.unwrap();
    sensor.fifo_stop_on_wtm_set(FifoEvent::Wtm).await.unwrap();
    sensor.timestamp_set(1).await.unwrap();

    // Configure interrupt pins
    let int1_route = PinInt1Route {
        int_on_res: 1,
        boot: 1,
        fifo_th: 1,
        fifo_ovr: 1,
        fifo_full: 1,
        ..Default::default()
    };

    sensor.pin_int1_route_set(&int1_route).await.unwrap();
    //sensor.pin_int2_route_set(&int1_route).await.unwrap();

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

        // Read number of samples in FIFO
        let mut num = sensor.fifo_data_level_get().await.unwrap() as u16;
        let mut ts: u32;
        writeln!(tx, "-- {} in FIFO", num).unwrap();
        while num > 0 {
            let curr_entry = (NUM_FIFO_ENTRY as u16) - num;
            num -= 1;
            let fdata = sensor.fifo_data_get(&md, &fifo_mode).await.unwrap();
            match FifoSensorTag::try_from(fdata.tag).unwrap_or_default() {
                FifoSensorTag::XlOnly2xTag => {
                    writeln!(
                        tx,
                        "{:2}_0: Acceleration [0][mg]:\t{:4.2}\t{:4.2}\t{:4.2}",
                        curr_entry, fdata.xl[0].mg[0], fdata.xl[0].mg[1], fdata.xl[0].mg[2]
                    )
                    .unwrap();
                    writeln!(
                        tx,
                        "{:2}_1: Acceleration [1][mg]:\t{:4.2}\t{:4.2}\t{:4.2}",
                        curr_entry, fdata.xl[1].mg[0], fdata.xl[1].mg[1], fdata.xl[1].mg[2]
                    )
                    .unwrap();
                }
                FifoSensorTag::XlTempTag => {
                    if fifo_mode.xl_only == 0 {
                        writeln!(
                            tx,
                            "{:2}: Acceleration [0][mg]:{:4.2}\t{:4.2}\t{:4.2}\tTemp[degC]:{:3.2}",
                            curr_entry,
                            fdata.xl[0].mg[0],
                            fdata.xl[0].mg[1],
                            fdata.xl[0].mg[2],
                            fdata.heat.deg_c
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            tx,
                            "{:2}: Acceleration [0][mg]:{:4.2}\t{:4.2}\t{:4.2}",
                            curr_entry, fdata.xl[0].mg[0], fdata.xl[0].mg[1], fdata.xl[0].mg[2]
                        )
                        .unwrap();
                    }
                }
                FifoSensorTag::TimestampTag => {
                    ts = fdata.cfg_chg.timestamp / 100;
                    writeln!(tx, "Timestamp:\t{} ms", ts).unwrap();
                }
                _ => {
                    writeln!(
                        tx,
                        "unknown TAG ({:#02x})",
                        FifoSensorTag::try_from(fdata.tag).unwrap_or_default() as u8
                    )
                    .unwrap();
                }
            }
        }

        writeln!(tx, "").unwrap();
    }
}
