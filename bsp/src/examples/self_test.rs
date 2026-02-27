use defmt::info;
use maybe_async::maybe_async;
use crate::*;
use libm::fabsf;
use iis2dulpx::*;
use iis2dulpx::prelude::*;
use core::fmt::{self, Debug, Display};

#[repr(u8)]
#[derive(PartialEq, Debug)]
enum StTestType {
    StPos = 0,
    StNeg = 1,
}

#[repr(u8)]
#[derive(PartialEq)]
enum StResult {
    StPass = 1,
    StFail = 0,
}

impl Display for StTestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StTestType::StPos => core::write!(f, "\nPOS"),
            StTestType::StNeg => core::write!(f, "NEG"),
        }
    }
}

const ST_RANGE_DEV_X_MIN: u16 = 50;
const ST_RANGE_DEV_X_MAX: u16 = 700;
const ST_RANGE_DEV_Y_MIN: u16 = 50;
const ST_RANGE_DEV_Y_MAX: u16 = 700;
const ST_RANGE_DEV_Z_MIN: u16 = 200;
const ST_RANGE_DEV_Z_MAX: u16 = 1200;

#[maybe_async]
async fn st_avg_5_samples<B, T>(
    sensor: &mut Iis2dulpx<B, T, MainBank>,
    md: &Md,
    fmode: &FifoMode,
) -> Result<FifoData, Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let mut fdata = FifoData::default();

    for _ in 0..5 {
        let tmp = sensor.fifo_data_get(md, fmode).await?;

        for j in 0..3 {
            fdata.xl[0].mg[j] += tmp.xl[0].mg[j];
        }
    }

    for j in 0..3 {
        fdata.xl[0].mg[j] /= 5.0;
    }

    Ok(fdata)
}

#[maybe_async]
pub async fn run<B, D, L>(bus: B, mut tx: L, mut delay: D, _int_pin: ()) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
{

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

    /*
     * Accelerometer Self Test
     */
    loop {
        for test in [StTestType::StPos, StTestType::StNeg] {
            /*
             * 1. Set the device in soft power-down and wait 10ms.
             */
            let mut md = Md {
                fs: Fs::_8g,
                bw: Bw::OdrDiv2,
                odr: Odr::Off,
            };
            sensor.mode_set(&md).await.unwrap();
            delay.delay_ms(10).await;

            /*
             * 2. Set the FIFO_EN bit in the CTRL4 (13h) register to 1.
             * 3. Set the XL_ONLY_FIFO bit in the FIFO_WTM (16h) register to 1.
             * 5. Set the FIFO_CTRL (15h) register to 00h to empty the FIFO
             */
            let mut fifo_mode = sensor.fifo_mode_get().await.unwrap();
            fifo_mode.operation = FifoOperation::BypassMode;
            fifo_mode.xl_only = 1;
            fifo_mode.store = Store::Fifo1x;
            let _ = sensor.fifo_mode_set(&fifo_mode).await;

            /*
             * 4. Set the ST_SIGN_X and ST_SIGN_Y bits in the CTRL3 (12h) register to 1
             *    and the ST_SIGN_Z bit in the WAKE_UP_DUR (1Dh) register to 0
             *    (i.e. 001 for POSITIVE. Instead, for NEGATIVE is 010).
             */
            if test == StTestType::StPos {
                sensor.self_test_sign_set(XlSelfTest::Positive).await.unwrap();
            } else {
                sensor.self_test_sign_set(XlSelfTest::Negative).await.unwrap();
            }

            /*
             * 6. Set ST[1:0] to "10"
             */
            sensor.self_test_start(2).await.unwrap();

            /*
             * 7. Set ODR = 200 hz, BW = ODR/2, FS = +/-8 g from the CTRL5 (14h) register
             *    and wait 50ms.
             */
            md.fs = Fs::_8g;
            md.bw = Bw::OdrDiv2;
            md.odr = Odr::_200hzLp;
            sensor.mode_set(&md).await.unwrap();
            delay.delay_ms(50).await;

            /*
             * 8. Set the FIFO_CTRL (15h) register to 01h to start filling the FIFO.
             */
            let mut fifo_mode = sensor.fifo_mode_get().await.unwrap();
            fifo_mode.operation = FifoOperation::FifoMode;
            sensor.fifo_mode_set(&fifo_mode).await.unwrap();

            /*
             * 9. Read the first 5 samples from FIFO, compute the average for each
             *    axis, and save the result in OUT1.
             */
            let mut num = 0;
            while num < 5 {
                num = sensor.fifo_data_level_get().await.unwrap();
            }

            let fdata1 = st_avg_5_samples(&mut sensor, &md, &fifo_mode).await.unwrap();

            /*
             * 10. Set device in Power Down mode and wait 10 ms.
             */
            let md = Md {
                fs: Fs::_8g,
                bw: Bw::OdrDiv2,
                odr: Odr::Off,
            };
            sensor.mode_set(&md).await.unwrap();
            delay.delay_ms(10).await;

            /*
             * 11. Set the FIFO_CTRL (15h) register to 00h to empty the FIFO.
             */
            let mut fifo_mode = sensor.fifo_mode_get().await.unwrap();
            fifo_mode.operation = FifoOperation::BypassMode;
            let _ = sensor.fifo_mode_set(&fifo_mode).await;

            /*
             * 12. Set ST[1:0] to "01"
             */
            let _ = sensor.self_test_start(1).await;

            /*
             * 13. Set ODR = 200 hz, BW = ODR/2, FS = +/-8 g from the CTRL5 (14h) register
             *     and wait 50ms.
             */
            let md = Md {
                fs: Fs::_8g,
                bw: Bw::OdrDiv2,
                odr: Odr::_200hzLp,
            };
            sensor.mode_set(&md).await.unwrap();
            delay.delay_ms(50).await;

            /*
             * 14. Set the FIFO_CTRL (15h) register to 01h to start filling the FIFO
             *     and wait 25ms.
             */
            let mut fifo_mode = sensor.fifo_mode_get().await.unwrap();
            fifo_mode.operation = FifoOperation::FifoMode;
            let _ = sensor.fifo_mode_set(&fifo_mode).await;
            delay.delay_ms(25).await;

            /*
             * 15. Read the first 5 samples from FIFO, compute the average for each
             *     axis, and save the result in OUT2.
             */
            num = 0;
            while num < 5 {
                num = sensor.fifo_data_level_get().await.unwrap();
            }

            let fdata2 = st_avg_5_samples(&mut sensor, &md, &fifo_mode).await.unwrap();

            /*
             * 16. Set device in Power Down mode and wait 10 ms.
             */
            let md = Md {
                fs: Fs::_8g,
                bw: Bw::OdrDiv2,
                odr: Odr::Off,
            };
            sensor.mode_set(&md).await.unwrap();
            delay.delay_ms(10).await;

            /*
             * 17. Set the ST[1:0] bits in the SELF_TEST (32h) register to 00.
             */
            sensor.self_test_stop().await.unwrap();

            /*
             * 18. Self-test deviation is 2 * |OUT2 - OUT1|. Compute the value for
             * each axis and verify that it falls within the range provided in the
             * datasheet
             */
            let mut st_dev = FifoData::default();
            for i in 0..3 {
                st_dev.xl[0].mg[i] = 2.0 * fabsf(fdata2.xl[0].mg[i] - fdata1.xl[0].mg[i]);
            }
            /*
             * 19. Set device in Power Down mode
             */
            let md = Md {
                fs: Fs::_8g,
                bw: Bw::OdrDiv2,
                odr: Odr::Off,
            };
            sensor.mode_set(&md).await.unwrap();

            /* check if stdev falls into given ranges */
            let mut st_result = StResult::StFail;
            if (st_dev.xl[0].mg[0] >= ST_RANGE_DEV_X_MIN.into()
                && st_dev.xl[0].mg[0] <= ST_RANGE_DEV_X_MAX.into())
                && (st_dev.xl[0].mg[1] >= ST_RANGE_DEV_Y_MIN.into()
                    && st_dev.xl[0].mg[1] <= ST_RANGE_DEV_Y_MAX.into())
                && (st_dev.xl[0].mg[2] >= ST_RANGE_DEV_Z_MIN.into()
                    && st_dev.xl[0].mg[2] <= ST_RANGE_DEV_Z_MAX.into())
            {
                st_result = StResult::StPass;
            }

            if st_result == StResult::StPass {
                writeln!(tx, "{} Self Test - PASS", test).unwrap();
            } else {
                writeln!(tx, "{} Self Test - FAIL!!!!", test).unwrap();
            }
        }
    }
}
