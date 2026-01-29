use crate::{BusOperation, DelayNs, Error, Iis2dulpx, PROPERTY_ENABLE};
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use st_mem_bank_macro::register;

/// Represents the register addresses of the device.
///
/// This enum is used to specify the addresses of various registers within the device, allowing for
/// read and write operations as specified.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Reg {
    /// Address for the `EXT_CLK_CFG` register (R/W).
    ExtClkCfg = 0x08,
    /// Address for the `PIN_CTRL` register (R/W).
    PinCtrl = 0x0C,
    /// Address for the `WAKE_UP_DUR_EXT` register (R/W).
    WakeUpDurExt = 0x0E,
    /// Address for the `WHO_AM_I` register (R).
    WhoAmI = 0x0F,
    /// Address for the `CTRL1` register (R/W).
    Ctrl1 = 0x10,
    /// Address for the `CTRL2` register (R/W).
    Ctrl2 = 0x11,
    /// Address for the `CTRL3` register (R/W).
    Ctrl3 = 0x12,
    /// Address for the `CTRL4` register (R/W).
    Ctrl4 = 0x13,
    /// Address for the `CTRL5` register (R/W).
    Ctrl5 = 0x14,
    /// Address for the `FIFO_CTRL` register (R/W).
    FifoCtrl = 0x15,
    /// Address for the `FIFO_WTM` register (R/W).
    FifoWtm = 0x16,
    /// Address for the `INTERRUPT_CFG` register (R/W).
    InterruptCfg = 0x17,
    /// Address for the `SIXD` register (R/W).
    Sixd = 0x18,
    /// Address for the `WAKE_UP_THS` register (R/W).
    WakeUpThs = 0x1C,
    /// Address for the `WAKE_UP_DUR` register (R/W).
    WakeUpDur = 0x1D,
    /// Address for the `FREE_FALL` register (R/W).
    FreeFall = 0x1E,
    /// Address for the `MD1_CFG` register (R/W).
    Md1Cfg = 0x1F,
    /// Address for the `MD1_CFG` register (R/W).
    Md2Cfg = 0x20,
    /// Address for the `WAKE_UP_SRC` register (R).
    WakeUpSrc = 0x21,
    /// Address for the `TAP_SRC` register (R).
    TapSrc = 0x22,
    /// Address for the `SIXD_SRC` register (R).
    SixdSrc = 0x23,
    /// Address for the `ALL_INT_SRC` register (R).
    AllIntSrc = 0x24,
    /// Address for the `STATUS` register (R).
    Status = 0x25,
    /// Address for the `FIFO_STATUS1` register (R).
    FifoStatus1 = 0x26,
    /// Address for the `FIFO_STATUS2` register (R).
    FifoStatus2 = 0x27,
    /// Address for the `OUT_X_L` register (R).
    OutXL = 0x28,
    /// Address for the `OUT_X_H` register (R).
    OutXH = 0x29,
    /// Address for the `OUT_Y_L` register (R).
    OutYL = 0x2A,
    /// Address for the `OUT_Y_H` register (R).
    OutYH = 0x2B,
    /// Address for the `OUT_Z_L` register (R).
    OutZL = 0x2C,
    /// Address for the `OUT_Z_H` register (R).
    OutZH = 0x2D,
    /// Address for the `OUT_T_AH_QVAR_L` register (R).
    OutTAhQvarL = 0x2E,
    /// Address for the `OUT_T_AH_QVAR_H` register (R).
    OutTAhQvarH = 0x2F,
    /// Address for the `AH_QVAR_CFG` register (R/W).
    AhQvarCfg = 0x31,
    /// Address for the `SELF_TEST` register (R/W).
    SelfTest = 0x32,
    /// Address for the `I3C_IF_CTRL` register (R/W).
    I3cIfCtrl = 0x33,
    /// Address for the `EMB_FUNC_STATUS_MAINPAGE` register (R).
    EmbFuncStatusMainpage = 0x34,
    /// Address for the `FSM_STATUS_MAINPAGE` register (R).
    FsmStatusMainpage = 0x35,
    /// Address for the `MLC_STATUS_MAINPAGE` register (R).
    MlcStatusMainpage = 0x36,
    /// Address for the `SLEEP` register (R/W).
    Sleep = 0x3D,
    /// Address for the `EN_DEVICE_CONFIG` register (W).
    EnDeviceConfig = 0x3E,
    /// Address for the `FUNC_CFG_ACCESS` register (R/W).
    FuncCfgAccess = 0x3F,
    /// Address for the `FIFO_DATA_OUT_TAG` register (R).
    FifoDataOutTag = 0x40,
    /// Address for the `FIFO_DATA_OUT_X_L` register (R).
    FifoDataOutXL = 0x41,
    /// Address for the `FIFO_DATA_OUT_X_H` register (R).
    FifoDataOutXH = 0x42,
    /// Address for the `FIFO_DATA_OUT_Y_L` register (R).
    FifoDataOutYL = 0x43,
    /// Address for the `FIFO_DATA_OUT_Y_H` register (R).
    FifoDataOutYH = 0x44,
    /// Address for the `FIFO_DATA_OUT_Z_L` register (R).
    FifoDataOutZL = 0x45,
    /// Address for the `FIFO_DATA_OUT_Z_H` register (R).
    FifoDataOutZH = 0x46,
    /// Address for the `FIFO_BATCH_DEC` register (R/W).
    FifoBatchDec = 0x47,
    /// Address for the `TAP_CFG0` register (R/W).
    TapCfg0 = 0x6F,
    /// Address for the `TAP_CFG1` register (R/W).
    TapCfg1 = 0x70,
    /// Address for the `TAP_CFG2` register (R/W).
    TapCfg2 = 0x71,
    /// Address for the `TAP_CFG3` register (R/W).
    TapCfg3 = 0x72,
    /// Address for the `TAP_CFG4` register (R/W).
    TapCfg4 = 0x73,
    /// Address for the `TAP_CFG5` register (R/W).
    TapCfg5 = 0x74,
    /// Address for the `TAP_CFG6` register (R/W).
    TapCfg6 = 0x75,
    /// Address for the `TIMESTAMP0` register (R).
    Timestamp0 = 0x7A,
    /// Address for the `TIMESTAMP1` register (R).
    Timestamp1 = 0x7B,
    /// Address for the `TIMESTAMP2` register (R).
    Timestamp2 = 0x7C,
    /// Address for the `TIMESTAMP3` register (R).
    Timestamp3 = 0x7D,
}

/// External Clock Configuration Register (R/W).
///
/// The `EXT_CLK_CFG` register is used to configure the external clock settings. It allows the external clock to replace the internal oscillator when enabled.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::ExtClkCfg, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct ExtClkCfg {
    #[bits(7, access = RO, default = 0)]
    not_used0: u8,

    /// External Clock Enable.
    ///
    /// When set to 1, the external clock replaces the internal oscillator.
    #[bits(1)]
    pub ext_clk_en: u8,
}

/// Pin Control Register (R/W).
///
/// The `PIN_CTRL` register configures the pin settings, including pull-up/pull-down resistors and SPI mode.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::PinCtrl, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PinCtrl {
    /// SPI Mode Selection.
    ///
    /// 0: 4-wire SPI (default); 1: 3-wire SPI.
    #[bits(1)]
    pub sim: u8,

    /// Push-Pull/Open-Drain Mode for INT Pins.
    ///
    /// 0: Push-pull mode (default); 1: Open-drain mode.
    #[bits(1)]
    pub pp_od: u8,

    /// CS Pin Pull-Up Disable.
    ///
    /// If 1, disables the internal pull-up of the CS pin.
    #[bits(1)]
    pub cs_pu_dis: u8,

    /// Interrupt Active Level.
    ///
    /// 0: Interrupts active-high (default); 1: Interrupts active-low.
    #[bits(1)]
    pub h_lactive: u8,

    /// INT1 Pin Pull-Down Disable.
    ///
    /// If 1, disables the internal pull-down of the INT1 pin.
    #[bits(1)]
    pub pd_dis_int1: u8,

    /// INT2 Pin Pull-Down Disable.
    ///
    /// If 1, disables the internal pull-down of the INT2 pin.
    #[bits(1)]
    pub pd_dis_int2: u8,

    /// SDA/SDI/SDO Pin Pull-Up Enable.
    ///
    /// If 1, enables the internal pull-up of the SDA/SDI/SDO pin.
    #[bits(1)]
    pub sda_pu_en: u8,

    /// SDO/TA0 Pin Pull-Up Enable.
    ///
    /// If 1, enables the internal pull-up of the SDO/TA0 pin.
    #[bits(1)]
    pub sdo_pu_en: u8,
}

/// Wake-Up Duration Extended Register (R/W).
///
/// The `WAKE_UP_DUR_EXT` register is used to select the resolution of the wake-up duration bits.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpDurExt, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpDurExt {
    #[bits(4, access = RO, default = 0)]
    not_used0: u8,

    /// Wake-Up Duration Extended.
    ///
    /// This bit is used to select the resolution of WAKE_DUR[1:0] bits in the WAKE_UP_DUR register.
    #[bits(1)]
    pub wu_dur_extended: u8,

    #[bits(3, access = RO, default = 0)]
    not_used1: u8,
}

/// Control Register 1 (R/W).
///
/// The `CTRL1` register configures various control settings, including wake-up event detection and data-ready mode.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl1, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl1 {
    /// Wake-Up Event Detection on Z-Axis.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub wu_z_en: u8,

    /// Wake-Up Event Detection on Y-Axis.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub wu_y_en: u8,

    /// Wake-Up Event Detection on X-Axis.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub wu_x_en: u8,

    /// Data-Ready Pulsed Mode.
    ///
    /// 0: Latched mode (default); 1: Pulsed mode.
    #[bits(1)]
    pub drdy_pulsed: u8,

    /// Register Address Auto-Increment.
    ///
    /// 0: Disabled; 1: Enabled (default).
    #[bits(1)]
    pub if_add_inc: u8,

    /// Software Reset.
    ///
    /// 0: Disabled; 1: Enabled. Automatically resets to 0 after the procedure.
    #[bits(1)]
    pub sw_reset: u8,

    /// Interrupt Routing on RES/EXT_CLK Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int1_on_res: u8,

    /// Smart Power Management Enable.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub smart_power_en: u8,
}

/// Control Register 2 (R/W).
///
/// The `CTRL2` register configures interrupt settings for the INT1 pin.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl2, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl2 {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// Data-Ready Interrupt on INT1 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int1_drdy: u8,

    /// FIFO Overrun Interrupt on INT1 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int1_fifo_ovr: u8,

    /// FIFO Threshold Interrupt on INT1 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int1_fifo_th: u8,

    /// FIFO Full Interrupt on INT1 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int1_fifo_full: u8,

    /// Boot Status on INT1 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int1_boot: u8,
}

#[register(address = Reg::Ctrl3, access_type = Iis2dulpx, generics = 2)]
/// Control Register 3 (R/W).
///
/// The `CTRL3` register configures interrupt settings for the INT2 pin and self-test configurations.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl3 {
    /// Self-Test Sign for X-Axis.
    ///
    /// 1: Positive; 0: Negative.
    #[bits(1)]
    pub st_sign_x: u8,

    /// Self-Test Sign for Y-Axis.
    ///
    /// 1: Positive; 0: Negative.
    #[bits(1)]
    pub st_sign_y: u8,

    /// High-Performance Mode Enable.
    ///
    /// 0: Low-power mode; 1: High-performance mode.
    #[bits(1)]
    pub hp_en: u8,

    /// Data-Ready Interrupt on INT2 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int2_drdy: u8,

    /// FIFO Overrun Interrupt on INT2 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int2_fifo_ovr: u8,

    /// FIFO Threshold Interrupt on INT2 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int2_fifo_th: u8,

    /// FIFO Full Interrupt on INT2 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int2_fifo_full: u8,

    /// Boot Status on INT2 Pin.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub int2_boot: u8,
}

/// Control Register 4 (R/W).
///
/// The `CTRL4` register configures various control settings, including FIFO and embedded functions enable.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl4, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl4 {
    /// Reboot Memory Content.
    ///
    /// 0: Normal mode; 1: Reboot memory content.
    #[bits(1)]
    pub boot: u8,

    /// Start of Conversion.
    ///
    /// 0: Normal mode; 1: Start conversion.
    #[bits(1)]
    pub soc: u8,

    #[bits(1, access = RO, default = 0)]
    not_used0: u8,

    /// FIFO Enable.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub fifo_en: u8,

    /// Embedded Functions Enable.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub emb_func_en: u8,

    /// Block Data Update.
    ///
    /// 0: Continuous update (default); 1: Output registers not updated until both MSByte and LSByte are read.
    #[bits(1)]
    pub bdu: u8,

    /// Inactivity ODR Selection.
    ///
    /// Selects the accelerometer ODR during inactivity status.
    #[bits(2)]
    pub inact_odr: u8,
}

/// Control Register 5 (R/W).
///
/// The `CTRL5` register configures the output data rate, bandwidth, and full-scale settings.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl5, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl5 {
    /// Full Scale Selection.
    ///
    /// Sets the full scale of the accelerometer.
    #[bits(2)]
    pub fs: u8,

    /// Bandwidth Selection.
    ///
    /// Selects the bandwidth of the device.
    #[bits(2)]
    pub bw: u8,

    /// Output Data Rate Selection.
    ///
    /// Sets the output data rate of the device.
    #[bits(4)]
    pub odr: u8,
}

/// FIFO Control Register (R/W).
///
/// The `FIFO_CTRL` register configures the FIFO settings, including mode, depth, and configuration change enable.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoCtrl, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl {
    /// FIFO Mode Selection.
    ///
    /// Different FIFO modes are enabled as shown in the datasheet.
    #[bits(3)]
    pub fifo_mode: u8,

    /// Stop on FIFO Threshold.
    ///
    /// 0: FIFO depth is not limited (default); 1: FIFO depth is limited to threshold level.
    #[bits(1)]
    pub stop_on_fth: u8,

    #[bits(1, access = RO, default = 0)]
    not_used0: u8,

    /// Disable Hard Reset on CS Pin.
    ///
    /// If 0, resetting the CS pin is equivalent to performing a deep power-off command. If 1, resetting the CS pin has no effect.
    #[bits(1)]
    pub dis_hard_rst_cs: u8,

    /// FIFO Depth Mode.
    ///
    /// If 1, enables 2x depth mode for FIFO buffer.
    #[bits(1)]
    pub fifo_depth: u8,

    /// Configuration Change Enable.
    ///
    /// Enables batching in FIFO of the device configuration and timestamp value when the ODR or BDR changes.
    #[bits(1)]
    pub cfg_chg_en: u8,
}

/// FIFO Watermark Register (R/W).
///
/// The `FIFO_WTM` register sets the FIFO watermark threshold and data configuration.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoWtm, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoWtm {
    /// FIFO Watermark Threshold.
    ///
    /// Maximum value is 127.
    #[bits(7)]
    pub fth: u8,

    /// Accelerometer Only FIFO.
    ///
    /// 0: Accelerometer and temperature/AH/Qvar data are stored in FIFO (default); 1: Only accelerometer data are stored in FIFO.
    #[bits(1)]
    pub xl_only_fifo: u8,
}

/// Interrupt Configuration Register (R/W).
///
/// The `INTERRUPT_CFG` register configures interrupt settings, including enabling interrupts and timestamp counter.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::InterruptCfg, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct InterruptCfg {
    /// Interrupts Enable.
    ///
    /// Enables basic interrupts (6D/4D, free-fall, wake-up, single/double/triple-tap, activity/inactivity).
    #[bits(1)]
    pub interrupts_enable: u8,

    /// Latched Interrupt Mode.
    ///
    /// 0: Interrupt level mode; 1: Interrupt latched mode.
    #[bits(1)]
    pub lir: u8,

    /// Disable Reset of Interrupt Flags.
    ///
    /// If 1, disables the reset of the interrupt flags when ALL_INT_SRC is read.
    #[bits(1)]
    pub dis_rst_lir_all_int: u8,

    /// Sleep Status on INT Pins.
    ///
    /// Sends the sleep status instead of sleep change to INT pins.
    #[bits(1)]
    pub sleep_status_on_int: u8,

    #[bits(1, access = RO, default = 0)]
    not_used0: u8,

    /// Wake-Up Threshold Weight.
    ///
    /// 0: 1 LSB = FS_XL / (2^6); 1: 1 LSB = FS_XL / (2^8).
    #[bits(1)]
    pub wake_ths_w: u8,

    #[bits(1, access = RO, default = 0)]
    not_used1: u8,

    /// Timestamp Enable.
    ///
    /// Enables timestamp counter. The counter is readable in TIMESTAMP0, TIMESTAMP1, TIMESTAMP2, and TIMESTAMP3.
    #[bits(1)]
    pub timestamp_en: u8,
}

/// 6D/4D Configuration Register (R/W).
///
/// The `SIXD` register configures the thresholds and enables 4D orientation detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Sixd, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Sixd {
    #[bits(5, access = RO, default = 0)]
    not_used0: u8,

    /// 6D/4D Thresholds.
    ///
    /// Sets the thresholds for 4D/6D function.
    #[bits(2)]
    pub d6d_ths: u8,

    /// 4D Orientation Detection Enable.
    ///
    /// 0: Disabled; 1: Enabled.
    #[bits(1)]
    pub d4d_en: u8,
}

/// Wake-Up Threshold Register (R/W).
///
/// The `WAKE_UP_THS` register sets the threshold for wake-up detection and enables activity/inactivity function.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpThs, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpThs {
    /// Wake-Up Threshold.
    ///
    /// Threshold for wake-up detection. 1 LSB weight depends on WAKE_THS_W in INTERRUPT_CFG.
    #[bits(6)]
    pub wk_ths: u8,

    /// Activity/Inactivity Function Enable.
    ///
    /// If 1, activity/inactivity function is enabled.
    #[bits(1)]
    pub sleep_on: u8,

    #[bits(1, access = RO, default = 0)]
    not_used0: u8,
}

/// Wake-Up Duration Register (R/W).
///
/// The `WAKE_UP_DUR` register configures the duration settings for sleep and wake-up events, as well as the self-test sign for the Z-axis.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpDur, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpDur {
    /// Sleep Duration.
    ///
    /// Duration to go into sleep mode. Default value: 0000 which corresponds to 16 ODR_time.
    #[bits(4)]
    pub sleep_dur: u8,

    /// Self-Test Sign for Z-Axis.
    ///
    /// Configures the sign of the self-test for the Z-axis.
    #[bits(1)]
    pub st_sign_z: u8,

    /// Wake-Up Duration.
    ///
    /// Duration for wake-up event detection.
    #[bits(2)]
    pub wake_dur: u8,

    /// Free-Fall Duration.
    ///
    /// Duration for free-fall event detection.
    #[bits(1)]
    pub ff_dur: u8,
}

/// Free-Fall Register (R/W).
///
/// The `FREE_FALL` register configures the threshold and duration for free-fall event detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FreeFall, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FreeFall {
    /// Free-Fall Threshold.
    ///
    /// Sets the threshold for free-fall detection.
    #[bits(3)]
    pub ff_ths: u8,

    /// Free-Fall Duration.
    ///
    /// Sets the duration for free-fall detection.
    #[bits(5)]
    pub ff_dur: u8,
}

/// MD1 Configuration Register (R/W).
///
/// The `MD1_CFG` register configures the interrupt signals routed to the INT1 pin.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Md1Cfg, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Md1Cfg {
    /// Embedded Functions Interrupt on INT1.
    ///
    /// Enables routing embedded functions event to the INT1 pin.
    #[bits(1)]
    pub int1_emb_func: u8,

    /// Timestamp Interrupt on INT1.
    ///
    /// Enables routing the alert of timestamp overflow to the INT1 pin.
    #[bits(1)]
    pub int1_timestamp: u8,

    /// 6D Recognition Interrupt on INT1.
    ///
    /// Enables routing 6D recognition event to the INT1 pin.
    #[bits(1)]
    pub int1_6d: u8,

    /// Tap Interrupt on INT1.
    ///
    /// Enables routing tap event to the INT1 pin.
    #[bits(1)]
    pub int1_tap: u8,

    /// Free-Fall Interrupt on INT1.
    ///
    /// Enables routing free-fall event to the INT1 pin.
    #[bits(1)]
    pub int1_ff: u8,

    /// Wake-Up Interrupt on INT1.
    ///
    /// Enables routing wake-up event to the INT1 pin.
    #[bits(1)]
    pub int1_wu: u8,

    #[bits(1, access = RO, default = 0)]
    not_used0: u8,

    /// Sleep Change Interrupt on INT1.
    ///
    /// Enables sleep change (or sleep status) on INT1 pin.
    #[bits(1)]
    pub int1_sleep_change: u8,
}

/// MD2 Configuration Register (R/W).
///
/// The `MD2_CFG` register configures the interrupt signals routed to the INT2 pin.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Md2Cfg, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Md2Cfg {
    /// Embedded Functions Interrupt on INT2.
    ///
    /// Enables routing embedded functions event to the INT2 pin.
    #[bits(1)]
    pub int2_emb_func: u8,

    /// Timestamp Interrupt on INT2.
    ///
    /// Enables routing the alert of timestamp overflow to the INT2 pin.
    #[bits(1)]
    pub int2_timestamp: u8,

    /// 6D Recognition Interrupt on INT2.
    ///
    /// Enables routing 6D recognition event to the INT2 pin.
    #[bits(1)]
    pub int2_6d: u8,

    /// Tap Interrupt on INT2.
    ///
    /// Enables routing tap event to the INT2 pin.
    #[bits(1)]
    pub int2_tap: u8,

    /// Free-Fall Interrupt on INT2.
    ///
    /// Enables routing free-fall event to the INT2 pin.
    #[bits(1)]
    pub int2_ff: u8,

    /// Wake-Up Interrupt on INT2.
    ///
    /// Enables routing wake-up event to the INT2 pin.
    #[bits(1)]
    pub int2_wu: u8,

    #[bits(1, access = RO, default = 0)]
    not_used0: u8,

    /// Sleep Change Interrupt on INT2.
    ///
    /// Enables sleep change (or sleep status) on INT2 pin.
    #[bits(1)]
    pub int2_sleep_change: u8,
}

/// Wake-Up Source Register (R).
///
/// The `WAKE_UP_SRC` register provides the status of wake-up events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpSrc, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpSrc {
    /// Z-Axis Wake-Up Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1)]
    pub z_wu: u8,

    /// Y-Axis Wake-Up Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1)]
    pub y_wu: u8,

    /// X-Axis Wake-Up Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1)]
    pub x_wu: u8,

    /// Wake-Up Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1)]
    pub wu_ia: u8,

    /// Sleep Status.
    ///
    /// 0: Activity status; 1: Inactivity status.
    #[bits(1)]
    pub sleep_state: u8,

    /// Free-Fall Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1)]
    pub ff_ia: u8,

    /// Sleep Change Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1)]
    pub sleep_change_ia: u8,

    #[bits(1, access = RO)]
    not_used0: u8,
}

/// Tap Source Register (R).
///
/// The `TAP_SRC` register provides the status of tap events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapSrc, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapSrc {
    #[bits(4, access = RO)]
    not_used0: u8,

    /// Triple-Tap Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub triple_tap_ia: u8,

    /// Double-Tap Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub double_tap_ia: u8,

    /// Single-Tap Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub single_tap_ia: u8,

    /// Tap Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub tap_ia: u8,
}

/// 6D Source Register (R).
///
/// The `SIXD_SRC` register provides the status of 6D/4D orientation events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::SixdSrc, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SixdSrc {
    /// X-Axis Low Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub xl: u8,

    /// X-Axis High Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub xh: u8,

    /// Y-Axis Low Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub yl: u8,

    /// Y-Axis High Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub yh: u8,

    /// Z-Axis Low Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub zl: u8,

    /// Z-Axis High Event.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub zh: u8,

    /// 6D/4D Orientation Change.
    ///
    /// 0: Not detected; 1: Detected.
    #[bits(1, access = RO)]
    pub d6d_ia: u8,

    #[bits(1, access = RO)]
    not_used0: u8,
}

/// All Interrupt Source Register (R).
///
/// The `ALL_INT_SRC` register provides the status of all interrupt events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::AllIntSrc, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct AllIntSrc {
    /// Free-Fall Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub ff_ia_all: u8,

    /// Wake-Up Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub wu_ia_all: u8,

    /// Single-Tap Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub single_tap_all: u8,

    /// Double-Tap Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub double_tap_all: u8,

    /// Triple-Tap Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub triple_tap_all: u8,

    /// 6D/4D Orientation Change Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub d6d_ia_all: u8,

    /// Sleep Change Interrupt Status.
    ///
    /// 0: Event not detected; 1: Event detected.
    #[bits(1, access = RO)]
    pub sleep_change_ia_all: u8,

    #[bits(1, access = RO)]
    not_used0: u8,
}

/// Status Register (R).
///
/// The `STATUS` register provides the status of data readiness and global interrupt events.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Status, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusRegister {
    /// Data Ready Status.
    ///
    /// 0: No new data; 1: New accelerometer data is available.
    #[bits(1, access = RO)]
    pub drdy: u8,

    #[bits(4, access = RO)]
    not_used0: u8,

    /// Global Interrupt Status.
    ///
    /// 0: No interrupt; 1: One of the following events has occurred: activity/inactivity status change, 6D/4D orientation change, tap event, wake-up event, free-fall event, or sleep event.
    #[bits(1, access = RO)]
    pub int_global: u8,

    #[bits(2, access = RO)]
    not_used1: u8,
}

/// FIFO Status Register 1 (R).
///
/// The `FIFO_STATUS1` register provides the status of FIFO events, including overrun and watermark status.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoStatus1, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoStatus1 {
    #[bits(6, access = RO)]
    not_used0: u8,

    /// FIFO Overrun Status.
    ///
    /// 0: No overrun; 1: FIFO has overwritten data.
    #[bits(1, access = RO)]
    pub fifo_ovr_ia: u8,

    /// FIFO Watermark Status.
    ///
    /// 0: FIFO filling is lower than WTM; 1: FIFO filling is equal to or higher than WTM.
    #[bits(1, access = RO)]
    pub fifo_wtm_ia: u8,
}

/// FIFO Status Register 2 (R).
///
/// The `FIFO_STATUS2` register provides the number of unread data stored in the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoStatus2, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoStatus2 {
    /// FIFO Sample Status.
    ///
    /// Number of unread data stored in FIFO.
    #[bits(8, access = RO)]
    pub fss: u8,
}

/// X-Axis Output Register (R).
///
/// The `OUT_X_L` and `OUT_X_H` registers provides the bits of the X-axis data output.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutXL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutX {
    /// X Data Output.
    #[bits(16, access = RO)]
    pub outx: u16,
}

/// Y-Axis Output Low Register (R).
///
/// The `OUT_Y_L` and `OUT_Y_H` registers provides the bits of the Y-axis data output.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutYL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutY {
    /// Y Data Output.
    #[bits(16, access = RO)]
    pub outy: u16,
}

/// Z-Axis Output Low Register (R).
///
/// The `OUT_Z_L` and `OUT_Z_H` registers provides the bits of the Z-axis data output.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutZL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutZ {
    /// Z Data Output.
    #[bits(16, access = RO)]
    pub outz: u16,
}

/// Temperature/AH/Qvar Output Low Register (R).
///
/// The `OUT_T_AH_QVAR_L` and  `OUT_T_AH_QVAR_H` registers provides the bits of the temperature/AH/Qvar data output.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutTAhQvarL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutTAhQvar {
    /// Temperature/AH/Qvar Data Output.
    #[bits(16, access = RO)]
    pub outt: u16,
}

/// AH/Qvar Configuration Register (R/W).
///
/// The `AH_QVAR_CFG` register configures the analog hub and Qvar settings, including gain, input impedance, and notch filter.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::AhQvarCfg, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct AhQvarCfg {
    #[bits(1, access = RO)]
    not_used0: u8,

    /// AH/Qvar Gain.
    ///
    /// Configures the input-output gain for the analog hub/Qvar.
    #[bits(2)]
    pub ah_qvar_gain: u8,

    /// AH/Qvar Input Impedance.
    ///
    /// Configures the equivalent input impedance of the analog hub/Qvar buffers.
    #[bits(2)]
    pub ah_qvar_c_zin: u8,

    /// AH/Qvar Notch Filter Cutoff Frequency.
    ///
    /// Selects the cutoff frequency of the notch filter.
    #[bits(1)]
    pub ah_qvar_notch_cutoff: u8,

    /// AH/Qvar Notch Filter Enable.
    ///
    /// Enables the notch filter in the analog hub/Qvar reading chain.
    #[bits(1)]
    pub ah_qvar_notch_en: u8,

    /// AH/Qvar Enable.
    ///
    /// Enables the analog hub and Qvar chain.
    #[bits(1)]
    pub ah_qvar_en: u8,
}

/// Self-Test Register (R/W).
///
/// The `SELF_TEST` register configures the self-test procedure and disables the temperature/Qvar acquisition chain.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::SelfTest, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SelfTest {
    /// Temperature/AH/Qvar Acquisition Chain Disable.
    ///
    /// 0: Acquisition chain enabled; 1: Acquisition chain disabled.
    #[bits(1)]
    pub t_ah_qvar_dis: u8,

    #[bits(3, access = RO)]
    not_used0: u8,

    /// Self-Test Enable.
    ///
    /// Enables data acquisition during the self-test procedure.
    #[bits(2)]
    pub st: u8,

    #[bits(2, access = RO)]
    not_used1: u8,
}

/// I3C Interface Control Register (R/W).
///
/// The `I3C_IF_CTRL` register configures the I3C interface settings, including bus activity selection and antispike filter.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::I3cIfCtrl, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct I3cIfCtrl {
    /// Bus Activity Selection.
    ///
    /// Selects the bus available time for in-band interrupt (IBI).
    #[bits(2)]
    pub bus_act_sel: u8,

    #[bits(3, access = RO)]
    not_used0: u8,

    /// Antispike Filter Enable.
    ///
    /// Enables the antispike filter even if the dynamic address is assigned.
    #[bits(1)]
    pub asf_on: u8,

    /// Disable Direct RSTDAA.
    ///
    /// 0: Direct RSTDAA supported; 1: Direct RSTDAA disabled.
    #[bits(1)]
    pub dis_drstdaa: u8,

    #[bits(1, access = RO)]
    not_used1: u8,
}

/// Embedded Function Status Main Page Register (R).
///
/// The `EMB_FUNC_STATUS_MAINPAGE` register provides the status of various embedded function events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::EmbFuncStatusMainpage, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncStatusMainpage {
    #[bits(3, access = RO)]
    not_used0: u8,

    /// Step Detection Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_step_det: u8,

    /// Tilt Detection Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_tilt: u8,

    /// Significant Motion Detection Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_sigmot: u8,

    #[bits(1, access = RO)]
    not_used1: u8,

    /// FSM Long Counter Timeout Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm_lc: u8,
}

/// FSM Status Main Page Register (R).
///
/// The `FSM_STATUS_MAINPAGE` register provides the status of FSM interrupt events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FsmStatusMainpage, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmStatusMainpage {
    /// FSM1 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm1: u8,

    /// FSM2 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm2: u8,

    /// FSM3 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm3: u8,

    /// FSM4 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm4: u8,

    /// FSM5 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm5: u8,

    /// FSM6 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm6: u8,

    /// FSM7 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm7: u8,

    /// FSM8 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_fsm8: u8,
}

/// MLC Status Main Page Register (R).
///
/// The `MLC_STATUS_MAINPAGE` register provides the status of MLC interrupt events detected by the sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::MlcStatusMainpage, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcStatusMainpage {
    /// MLC1 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_mlc1: u8,

    /// MLC2 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_mlc2: u8,

    /// MLC3 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_mlc3: u8,

    /// MLC4 Interrupt Status.
    ///
    /// 0: No interrupt; 1: Interrupt detected.
    #[bits(1, access = RO)]
    pub is_mlc4: u8,

    #[bits(4, access = RO)]
    not_used0: u8,
}

/// Sleep Register (R/W).
///
/// The `SLEEP` register configures the deep power-down state of the device.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Sleep, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Sleep {
    /// Deep Power-Down.
    ///
    /// If set to 1, the device enters a deep power-down state.
    #[bits(1)]
    pub deep_pd: u8,

    #[bits(7, access = RO, default = 0)]
    not_used0: u8,
}

/// Enable Device Configuration Register (W).
///
/// The `EN_DEVICE_CONFIG` register allows the transition from deep power-down to soft power-down when the SPI interface is used.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::EnDeviceConfig, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EnDeviceConfig {
    /// Soft Power-Down.
    ///
    /// This bit is write-only and allows the transition from deep power-down to soft power-down.
    #[bits(1, access = WO)]
    pub soft_pd: u8,

    #[bits(7, access = RO)]
    not_used0: u8,
}

/// Function Configuration Access Register (R/W).
///
/// The `FUNC_CFG_ACCESS` register enables access to embedded function registers and FSM control.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FuncCfgAccess, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FuncCfgAccess {
    /// FSM Write Control Enable.
    ///
    /// Enables the FSM to control the CTRL registers autonomously.
    #[bits(1)]
    pub fsm_wr_ctrl_en: u8,

    #[bits(6, access = RO)]
    not_used0: u8,

    /// Embedded Function Register Access.
    ///
    /// Enables access to the embedded functions registers.
    #[bits(1)]
    pub emb_func_reg_access: u8,
}

/// FIFO Data Output Tag Register (R).
///
/// The `FIFO_DATA_OUT_TAG` register contains the TAG values that distinguish the different kinds of data that can be batched in FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoDataOutTag, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutTag {
    #[bits(3, access = RO)]
    not_used0: u8,

    /// Sensor Tag.
    ///
    /// Identifies the sensor in FIFO data output.
    #[bits(5)]
    pub tag_sensor: u8,
}

/// FIFO Data Output X Low Register (R).
///
/// The `FIFO_DATA_OUT_X_L` register provides the least significant bits of the X-axis data output from the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutXL {
    /// FIFO X-Axis Data Output (LSBs).
    #[bits(8)]
    pub fifo_data_out: u8,
}

/// FIFO Data Output X High Register (R).
///
/// The `FIFO_DATA_OUT_X_H` register provides the most significant bits of the X-axis data output from the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutXH {
    /// FIFO X-Axis Data Output (MSBs).
    #[bits(8)]
    pub fifo_data_out: u8,
}

/// FIFO Data Output Y Low Register (R).
///
/// The `FIFO_DATA_OUT_Y_L` register provides the least significant bits of the Y-axis data output from the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutYL {
    /// FIFO Y-Axis Data Output (LSBs).
    #[bits(8)]
    pub fifo_data_out: u8,
}

/// FIFO Data Output Y High Register (R).
///
/// The `FIFO_DATA_OUT_Y_H` register provides the most significant bits of the Y-axis data output from the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutYH {
    /// FIFO Y-Axis Data Output (MSBs).
    #[bits(8)]
    pub fifo_data_out: u8,
}

/// FIFO Data Output Z Low Register (R).
///
/// The `FIFO_DATA_OUT_Z_L` register provides the least significant bits of the Z-axis data output from the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutZL {
    /// FIFO Z-Axis Data Output (LSBs).
    #[bits(8)]
    pub fifo_data_out: u8,
}

/// FIFO Data Output Z High Register (R).
///
/// The `FIFO_DATA_OUT_Z_H` register provides the most significant bits of the Z-axis data output from the FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutZH {
    /// FIFO Z-Axis Data Output (MSBs).
    #[bits(8)]
    pub fifo_data_out: u8,
}

/// FIFO Batch Decimation Register (R/W).
///
/// The `FIFO_BATCH_DEC` register configures the batch data rate for accelerometer data and the decimation for timestamp batching in FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoBatchDec, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoBatchDec {
    /// Batch Data Rate for Accelerometer.
    ///
    /// Selects the batch data rate (write frequency in FIFO) for accelerometer data.
    #[bits(3)]
    pub bdr_xl: u8,

    /// Decimation for Timestamp Batching.
    ///
    /// Selects decimation for timestamp batching in FIFO.
    #[bits(2)]
    pub dec_ts_batch: u8,

    #[bits(3, access = RO)]
    not_used0: u8,
}

/// Tap Configuration 0 Register (R/W).
///
/// The `TAP_CFG0` register configures the axis selection and inverted peak search for tap event detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg0, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg0 {
    #[bits(1, access = RO)]
    not_used0: u8,

    /// Inverted Peak Search.
    ///
    /// Enables the search of the inverted peak by selecting the maximum number of samples between the first and second (inverted) peak in tap detection.
    #[bits(5)]
    pub invert_t: u8,

    /// Axis Selection for Tap Event.
    ///
    /// Selection of axis for tap event research.
    #[bits(2)]
    pub axis: u8,
}

/// Tap Configuration 1 Register (R/W).
///
/// The `TAP_CFG1` register configures the thresholds and samples for stationary conditions before and after shock.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg1, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg1 {
    /// Post-Shock Stationary Samples.
    ///
    /// Number of samples during stationary condition after shock and wait phases.
    #[bits(4)]
    pub post_still_t: u8,

    /// Pre-Shock Stationary Threshold.
    ///
    /// Threshold for stationary condition before shock.
    #[bits(4)]
    pub pre_still_ths: u8,
}

/// Tap Configuration 2 Register (R/W).
///
/// The `TAP_CFG2` register configures the wait time for shock completion and post-shock stationary samples.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg2, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg2 {
    /// Wait Time for Shock Completion.
    ///
    /// Programs the number of samples to wait for the shock to finish.
    #[bits(6)]
    pub wait_t: u8,

    /// Post-Shock Stationary Samples.
    ///
    /// Number of samples during stationary condition after shock and wait phases.
    #[bits(2)]
    pub post_still_t: u8,
}

/// Tap Configuration 3 Register (R/W).
///
/// The `TAP_CFG3` register configures the latency and thresholds for stationary conditions after shock.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg3, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg3 {
    /// Latency for Consecutive Taps.
    ///
    /// Maximum number of samples between consecutive taps event to detect double or triple tap.
    #[bits(4)]
    pub latency_t: u8,

    /// Post-Shock Stationary Threshold.
    ///
    /// Threshold for stationary condition after shock and wait phases.
    #[bits(4)]
    pub post_still_ths: u8,
}

/// Tap Configuration 4 Register (R/W).
///
/// The `TAP_CFG4` register configures the peak detection threshold and latency window for tap events.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg4, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg4 {
    /// Peak Detection Threshold.
    ///
    /// Threshold for peak detection.
    #[bits(6)]
    pub peak_ths: u8,

    #[bits(1, access = RO)]
    not_used0: u8,

    /// Wait End of Latency.
    ///
    /// Enables the feature to wait for the end of the latency window to determine the tap event level.
    #[bits(1)]
    pub wait_end_latency: u8,
}

/// Tap Configuration 5 Register (R/W).
///
/// The `TAP_CFG5` register configures the rebound time and enables single, double, and triple tap events.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg5, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg5 {
    /// Rebound Time.
    ///
    /// Programs the number of samples to wait for the rebound to finish.
    #[bits(5)]
    pub rebound_t: u8,

    /// Single Tap Enable.
    ///
    /// Enables the single-tap event.
    #[bits(1)]
    pub single_tap_en: u8,

    /// Double Tap Enable.
    ///
    /// Enables the double-tap event.
    #[bits(1)]
    pub double_tap_en: u8,

    /// Triple Tap Enable.
    ///
    /// Enables the triple-tap event.
    #[bits(1)]
    pub triple_tap_en: u8,
}

/// Tap Configuration 6 Register (R/W).
///
/// The `TAP_CFG6` register configures the starting sample and number of samples for stationary conditions before shock.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapCfg6, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg6 {
    /// Pre-Shock Stationary Samples.
    ///
    /// Selection of number of samples for stationary condition before shock.
    #[bits(4)]
    pub pre_still_n: u8,

    /// Pre-Shock Stationary Starting Sample.
    ///
    /// Selection of starting sample for stationary condition before shock.
    #[bits(4)]
    pub pre_still_st: u8,
}

/// Timestamp Output Register 0-3 (R).
///
/// The `TIMESTAMP0-3` register provides the bytes of the timestamp output.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Timestamp0, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u32, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u32, order = Lsb))]
pub struct Timestamp {
    #[bits(32, access = RO)]
    pub timestamp: u32,
}

/// Represents the status of the device.
///
/// # Fields
///
/// - `sw_reset: u8`: Indicates the status of the software reset.
/// - `boot: u8`: Indicates the status of the boot process.
/// - `drdy: u8`: Indicates the data-ready status.
///
/// # Description
///
/// This struct encapsulates various status indicators of the device, providing information about
/// the software reset, boot process, data readiness, and power-down state.
pub struct Status {
    pub sw_reset: u8,
    pub boot: u8,
    pub drdy: u8,
}

/// Represents the sensor mode configuration.
///
/// # Fields
///
/// - `odr: Odr`: Specifies the output data rate (ODR).
/// - `fs: Fs`: Specifies the full-scale (FS) range.
/// - `bw: Bw`: Specifies the bandwidth (BW).
///
/// # Description
///
/// This struct encapsulates the sensor mode configuration, including settings for ODR, FS, and BW.
/// It is used to configure and retrieve the sensor's operational mode.
#[derive(Default)]
pub struct Md {
    pub odr: Odr,
    pub fs: Fs,
    pub bw: Bw,
}

/// Represents various source information from the device.
///
/// # Fields
///
/// - `drdy: u8`: Data-ready status.
/// - `timestamp: u8`: Timestamp (assumed 0).
/// - `free_fall: u8`: Free-fall detection status.
/// - `wake_up: u8`: Wake-up detection status.
/// - `wake_up_z: u8`: Wake-up detection on Z-axis.
/// - `wake_up_y: u8`: Wake-up detection on Y-axis.
/// - `wake_up_x: u8`: Wake-up detection on X-axis.
/// - `single_tap: u8`: Single tap detection status.
/// - `double_tap: u8`: Double tap detection status.
/// - `triple_tap: u8`: Triple tap detection status.
/// - `six_d: u8`: 6D detection status.
/// - `six_d_xl: u8`: 6D detection on X-axis low.
/// - `six_d_xh: u8`: 6D detection on X-axis high.
/// - `six_d_yl: u8`: 6D detection on Y-axis low.
/// - `six_d_yh: u8`: 6D detection on Y-axis high.
/// - `six_d_zl: u8`: 6D detection on Z-axis low.
/// - `six_d_zh: u8`: 6D detection on Z-axis high.
/// - `sleep_change: u8`: Sleep change status.
/// - `sleep_state: u8`: Sleep state status.
///
/// # Description
///
/// This struct encapsulates various source information from the device, including detection and
/// status indicators for events such as free-fall, wake-up, tap detection, 6D orientation, and FIFO status.
pub struct AllSources {
    pub drdy: u8,
    pub free_fall: u8,
    pub wake_up: u8,
    pub wake_up_z: u8,
    pub wake_up_y: u8,
    pub wake_up_x: u8,
    pub single_tap: u8,
    pub double_tap: u8,
    pub triple_tap: u8,
    pub six_d: u8,
    pub six_d_xl: u8,
    pub six_d_xh: u8,
    pub six_d_yl: u8,
    pub six_d_yh: u8,
    pub six_d_zl: u8,
    pub six_d_zh: u8,
    pub sleep_change: u8,
    pub sleep_state: u8,
}

/// Represents accelerometer data.
///
/// # Fields
///
/// - `mg: [f32; 3]`: Converted acceleration values in milli-g.
/// - `raw: [i16; 3]`: Raw acceleration data.
///
/// # Description
///
/// This struct encapsulates accelerometer data, providing both raw and converted values for each axis.
pub struct XlData {
    pub mg: [f32; 3],
    pub raw: [i16; 3],
}

/// Represents AH_QVAR data.
///
/// # Fields
///
/// - `mv: f32`: Converted voltage in millivolts.
/// - `raw: i16`: Raw AH_QVAR data.
///
/// # Description
///
/// This struct encapsulates AH_QVAR data, providing both raw and converted voltage values.
pub struct AhQvarData {
    pub mv: f32,
    pub raw: i16,
}

/// Represents the I3C configuration parameters.
///
/// # Fields
///
/// - `bus_act_sel: BusActSel`: Specifies the bus activity selection.
/// - `asf_on: u8`: Indicates if asynchronous frame support is enabled.
/// - `drstdaa_en: u8`: Indicates if dynamic address assignment is disabled.
///
/// # Description
///
/// This struct encapsulates the configuration parameters for the I3C bus, allowing customization of
/// bus activity, asynchronous frame support, and dynamic address assignment.
pub struct I3cCfg {
    pub bus_act_sel: BusActSel,
    pub asf_on: u8,
    pub drstdaa_en: u8,
}

/// Represents the electrical configuration for configurable pins.
///
/// # Fields
///
/// - `sdo_pull_up: u8`: Indicates if the SDO pin pull-up is enabled.
/// - `sda_pull_up: u8`: Indicates if the SDA pin pull-up is enabled.
/// - `cs_pull_up: u8`: Indicates if the CS pin pull-up is enabled.
/// - `int1_int2_push_pull: u8`: Indicates if INT1 and INT2 pins are configured as push-pull.
/// - `int1_pull_down: u8`: Indicates if the INT1 pin pull-down is enabled.
/// - `int2_pull_down: u8`: Indicates if the INT2 pin pull-down is enabled.
///
/// # Description
///
/// This struct encapsulates the electrical configuration for configurable pins, allowing customization
/// of pull-up and pull-down settings.
pub struct PinConf {
    pub sdo_pull_up: u8,
    pub sda_pull_up: u8,
    pub cs_pull_up: u8,
    pub int1_int2_push_pull: u8,
    pub int1_pull_down: u8,
    pub int2_pull_down: u8,
}

/// Represents the interrupt signals routing configuration for the INT1 pin.
///
/// # Fields
///
/// - `int_on_res: u8`: Indicates if the interrupt is routed on the resolution.
/// - `drdy: u8`: Data-ready interrupt.
/// - `boot: u8`: Boot interrupt.
/// - `fifo_th: u8`: FIFO threshold interrupt.
/// - `fifo_ovr: u8`: FIFO overrun interrupt.
/// - `fifo_full: u8`: FIFO full interrupt.
/// - `free_fall: u8`: Free-fall interrupt.
/// - `six_d: u8`: 6D orientation interrupt.
/// - `tap: u8`: Tap interrupt.
/// - `wake_up: u8`: Wake-up interrupt.
/// - `sleep_change: u8`: Sleep change interrupt.
/// - `emb_function: u8`: Embedded function interrupt.
/// - `timestamp: u8`: Timestamp interrupt.
///
/// # Description
///
/// This struct encapsulates the interrupt signals routing configuration for the INT1 pin, allowing
/// customization of various interrupt sources.
#[derive(Default)]
pub struct PinInt1Route {
    pub int_on_res: u8,
    pub drdy: u8,
    pub boot: u8,
    pub fifo_th: u8,
    pub fifo_ovr: u8,
    pub fifo_full: u8,
    pub free_fall: u8,
    pub six_d: u8,
    pub tap: u8,
    pub wake_up: u8,
    pub sleep_change: u8,
    pub emb_function: u8,
    pub timestamp: u8,
}

/// Represents the interrupt signals routing configuration for the INT2 pin.
///
/// # Fields
///
/// - `drdy: u8`: Data-ready interrupt.
/// - `boot: u8`: Boot interrupt.
/// - `fifo_th: u8`: FIFO threshold interrupt.
/// - `fifo_ovr: u8`: FIFO overrun interrupt.
/// - `fifo_full: u8`: FIFO full interrupt.
/// - `free_fall: u8`: Free-fall interrupt.
/// - `six_d: u8`: 6D orientation interrupt.
/// - `tap: u8`: Tap interrupt.
/// - `wake_up: u8`: Wake-up interrupt.
/// - `sleep_change: u8`: Sleep change interrupt.
/// - `emb_function: u8`: Embedded function interrupt.
/// - `timestamp: u8`: Timestamp interrupt.
///
/// # Description
///
/// This struct encapsulates the interrupt signals routing configuration for the INT2 pin, allowing
/// customization of various interrupt sources.
#[derive(Default)]
pub struct PinInt2Route {
    pub drdy: u8,
    pub boot: u8,
    pub fifo_th: u8,
    pub fifo_ovr: u8,
    pub fifo_full: u8,
    pub free_fall: u8,
    pub six_d: u8,
    pub tap: u8,
    pub wake_up: u8,
    pub sleep_change: u8,
    pub emb_function: u8,
    pub timestamp: u8,
}

/// Represents the interrupt configuration settings.
///
/// # Fields
///
/// - `int_cfg: IntCfg`: Specifies the interrupt mode (disabled, level-sensitive, or latched).
/// - `sleep_status_on_int: u8`: Indicates if sleep status is reported on interrupt.
/// - `dis_rst_lir_all_int: u8`: Indicates if reset on latch interrupt is disabled.
///
/// # Description
///
/// This struct encapsulates the interrupt configuration settings, allowing customization of the
/// interrupt mode and additional settings.
#[derive(Default)]
pub struct IntConfig {
    pub int_cfg: IntCfg,
    pub sleep_status_on_int: u8,
    pub dis_rst_lir_all_int: u8,
}

/// Represents the configuration settings for the AH_QVAR chain.
///
/// # Fields
///
/// - `ah_qvar_en: u8`: Indicates if the AH_QVAR chain is enabled.
/// - `ah_qvar_notch_en: u8`: Indicates if the notch filter is enabled.
/// - `ah_qvar_notch: AhQvarNotch`: Specifies the notch filter cutoff frequency.
/// - `ah_qvar_zin: AhQvarZin`: Specifies the input impedance.
/// - `ah_qvar_gain: AhQvarGain`: Specifies the gain setting.
///
/// # Description
///
/// This struct encapsulates the configuration settings for the AH_QVAR chain, allowing customization
/// of gain, input impedance, notch filter, and enabling the AH_QVAR chain.
#[derive(Default)]
pub struct AhQvarMode {
    pub ah_qvar_en: u8,
    pub ah_qvar_notch_en: u8,
    pub ah_qvar_notch: AhQvarNotch,
    pub ah_qvar_zin: AhQvarZin,
    pub ah_qvar_gain: AhQvarGain,
}

/// Represents the configuration settings for the 4D/6D detection function.
///
/// # Fields
///
/// - `threshold: Threshold`: The threshold for 4D/6D detection.
/// - `mode: Mode`: The mode of detection (4D or 6D).
///
/// # Description
///
/// This struct encapsulates the configuration settings for the 4D/6D detection function, allowing customization
/// of the detection mode and threshold.
#[derive(Default)]
pub struct SixdConfig {
    pub threshold: Threshold,
    pub mode: Mode,
}

/// Represents the configuration settings for the wakeup function.
///
/// # Fields
///
/// - `wake_dur: WakeDur`: The wake duration setting.
/// - `sleep_dur: u8`: The sleep duration setting.
/// - `wake_ths: u8`: The wake threshold setting.
/// - `wake_ths_weight: u8`: The weight of the wake threshold.
/// - `wake_enable: WakeEnable`: Indicates if the wakeup function is enabled.
/// - `inact_odr: InactOdr`: The inactivity ODR setting.
///
/// # Description
///
/// This struct encapsulates the configuration settings for the wakeup function, allowing customization
/// of wake duration, sleep duration, wake threshold, and inactivity ODR.
#[derive(Default)]
pub struct WakeupConfig {
    pub wake_dur: WakeDur,
    pub sleep_dur: u8,
    pub wake_ths: u8,
    pub wake_ths_weight: u8,
    pub wake_enable: WakeEnable,
    pub inact_odr: InactOdr,
}

/// Represents the configuration settings for tap detection.
///
/// # Fields
///
/// - `axis: Axis`: Specifies the axis for tap detection (X, Y, Z, or none).
/// - `inverted_peak_time: u8`: Inverted peak time for tap detection.
/// - `pre_still_ths: u8`: Pre-still threshold.
/// - `post_still_ths: u8`: Post-still threshold.
/// - `post_still_time: u8`: Post-still time.
/// - `shock_wait_time: u8`: Shock wait time.
/// - `latency: u8`: Latency time.
/// - `wait_end_latency: u8`: Wait end latency.
/// - `peak_ths: u8`: Peak threshold.
/// - `rebound: u8`: Rebound time.
/// - `pre_still_start: u8`: Pre-still start time.
/// - `pre_still_n: u8`: Pre-still N value.
/// - `single_tap_on: u8`: Enables single tap detection.
/// - `double_tap_on: u8`: Enables double tap detection.
/// - `triple_tap_on: u8`: Enables triple tap detection.
///
/// # Description
///
/// This struct encapsulates the configuration settings for tap detection, allowing customization of
/// axis selection, thresholds, timings, and enabling single, double, or triple tap detection.
pub struct TapConfig {
    pub axis: Axis,
    pub inverted_peak_time: u8,
    pub pre_still_ths: u8,
    pub post_still_ths: u8,
    pub post_still_time: u8,
    pub shock_wait_time: u8,
    pub latency: u8,
    pub wait_end_latency: u8,
    pub peak_ths: u8,
    pub rebound: u8,
    pub pre_still_start: u8,
    pub pre_still_n: u8,
    pub single_tap_on: u8,
    pub double_tap_on: u8,
    pub triple_tap_on: u8,
}

/// Represents OUTT data.
///
/// # Fields
///
/// - `heat: Heat`: Contains temperature data.
///
/// # Description
///
/// This struct encapsulates OUTT data, providing temperature information through a `Heat` struct.
#[derive(Default)]
pub struct OuttData {
    pub heat: Heat,
}

/// Represents temperature data.
///
/// # Fields
///
/// - `deg_c: f32`: Temperature in degrees Celsius.
/// - `raw: i16`: Raw temperature data.
///
/// # Description
///
/// This struct encapsulates temperature data, providing both raw and converted values.
#[derive(Default)]
pub struct Heat {
    pub deg_c: f32,
    pub raw: i16,
}

/// Represents the FIFO mode configuration.
///
/// # Fields
///
/// - `operation: FifoOperation`: Specifies the FIFO operation mode.
/// - `store: Store`: Specifies the storage depth (1X or 2X).
/// - `xl_only: u8`: Indicates if only XL samples (16-bit) are stored in the FIFO.
/// - `watermark: u8`: The FIFO watermark level (0 to disable).
/// - `cfg_change_in_fifo: u8`: Indicates if configuration changes are stored in the FIFO.
/// - `fifo_event: FifoEvent`: Specifies the FIFO event type (watermark or full).
/// - `batch: Batch`: Contains batching information for timestamp and accelerometer data rate.
///
/// # Description
///
/// This struct encapsulates the FIFO mode configuration, allowing customization of operation mode,
/// storage depth, watermark, and batching settings.
#[derive(Default)]
pub struct FifoMode {
    pub operation: FifoOperation,
    pub store: Store,
    pub xl_only: u8,
    pub cfg_change_in_fifo: u8,
}

/// Represents batching information for the FIFO.
///
/// # Fields
///
/// - `dec_ts: DecTs`: Specifies the decimation for timestamp batching.
/// - `bdr_xl: BdrXl`: Specifies the accelerometer batch data rate.
///
/// # Description
///
/// This struct encapsulates batching information for the FIFO, allowing customization of timestamp
/// decimation and accelerometer data rate.
#[derive(Default)]
pub struct Batch {
    pub dec_ts: DecTs,
    pub bdr_xl: BdrXl,
}

/// Represents the processed FIFO data.
///
/// # Fields
///
/// - `tag: u8`: The sensor tag.
/// - `xl: [Xl; 2]`: Accelerometer data for two samples.
/// - `ah_qvar: AhQvar`: AH_QVAR data.
/// - `heat: Heat`: Temperature data.
/// - `pedo: Pedo`: Pedometer data.
/// - `cfg_chg: CfgChg`: Configuration change data.
///
/// # Description
///
/// This struct encapsulates the processed FIFO data, including accelerometer, AH_QVAR, temperature,
/// pedometer, and configuration change information.
#[derive(Default)]
pub struct FifoData {
    pub tag: u8,
    pub xl: [Xl; 2],
    pub ah_qvar: AhQvar,
    pub heat: Heat,
    pub pedo: Pedo,
    pub cfg_chg: CfgChg,
}

/// Represents accelerometer data.
///
/// # Fields
///
/// - `mg: [f32; 3]`: Converted acceleration values in milli-g.
/// - `raw: [i16; 3]`: Raw acceleration data.
///
/// # Description
///
/// This struct encapsulates accelerometer data, providing both raw and converted values for each axis.
#[derive(Default)]
pub struct Xl {
    pub mg: [f32; 3],
    pub raw: [i16; 3],
}

/// Represents AH_QVAR data.
///
/// # Fields
///
/// - `mv: f32`: Converted voltage in millivolts.
/// - `raw: i16`: Raw AH_QVAR data.
///
/// # Description
///
/// This struct encapsulates AH_QVAR data, providing both raw and converted voltage values.
#[derive(Default)]
pub struct AhQvar {
    pub mv: f32,
    pub raw: i16,
}

/// Represents pedometer data.
///
/// # Fields
///
/// - `steps: u32`: The number of steps counted.
/// - `timestamp: u32`: The timestamp associated with the step count.
///
/// # Description
///
/// This struct encapsulates pedometer data, providing step count and timestamp information.
#[derive(Default)]
pub struct Pedo {
    pub steps: u32,
    pub timestamp: u32,
}

/// Represents configuration change data.
///
/// # Fields
///
/// - `cfg_change: u8`: Indicates if a configuration change occurred (1 bit).
/// - `odr: u8`: Output data rate (4 bits).
/// - `bw: u8`: Bandwidth (2 bits).
/// - `lp_hp: u8`: Low-power/high-performance mode (1 bit).
/// - `qvar_en: u8`: AH_QVAR enable status (1 bit).
/// - `fs: u8`: Full-scale range (2 bits).
/// - `dec_ts: u8`: Decimation for timestamp batching (2 bits).
/// - `odr_xl_batch: u8`: Output data rate for accelerometer batching (1 bit).
/// - `timestamp: u32`: The timestamp associated with the configuration change.
///
/// # Description
///
/// This struct encapsulates configuration change data, providing detailed information about changes
/// in sensor settings and associated timestamps.
#[derive(Default)]
pub struct CfgChg {
    pub cfg_change: u8,
    pub odr: u8,
    pub bw: u8,
    pub lp_hp: u8,
    pub qvar_en: u8,
    pub fs: u8,
    pub dec_ts: u8,
    pub odr_xl_batch: u8,
    pub timestamp: u32,
}

/// Represents the bus activity selection for the I3C interface.
///
/// # Variants
///
/// - `_20us`: Bus available time of 20 microseconds.
/// - `_50us`: Bus available time of 50 microseconds.
/// - `_1ms`: Bus available time of 1 millisecond.
/// - `_25ms`: Bus available time of 25 milliseconds.
///
/// # Description
///
/// This enum is used to specify the bus activity selection for the I3C interface, determining the
/// available time for bus operations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum BusActSel {
    #[default]
    _20us = 0x0,
    _50us = 0x1,
    _1ms = 0x2,
    _25ms = 0x3,
}

/// Represents the interrupt configuration modes.
///
/// # Variants
///
/// - `Disabled`: Interrupts are disabled.
/// - `Level`: Interrupts are level-sensitive.
/// - `Latched`: Interrupts are latched.
///
/// # Description
///
/// This enum is used to specify the interrupt configuration mode, allowing for disabled, level-sensitive,
/// or latched interrupt settings.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default)]
pub enum IntCfg {
    #[default]
    Disabled = 0x0,
    Level = 0x1,
    Latched = 0x2,
}

/// Represents the notch filter cutoff frequency options for the AH_QVAR chain.
///
/// # Variants
///
/// - `_50hz`: 50 Hz notch filter cutoff.
/// - `_60hz`: 60 Hz notch filter cutoff.
///
/// # Description
///
/// This enum is used to specify the notch filter cutoff frequency for the AH_QVAR chain.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum AhQvarNotch {
    _50hz = 0x0,
    #[default]
    _60hz = 0x1,
}

/// Represents the input impedance options for the AH_QVAR chain.
///
/// # Variants
///
/// - `_520mohm`: 520 MΩ input impedance.
/// - `_175mohm`: 175 MΩ input impedance.
/// - `_310mohm`: 310 MΩ input impedance.
/// - `_75mohm`: 75 MΩ input impedance.
///
/// # Description
///
/// This enum is used to specify the input impedance for the AH_QVAR chain.
#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, TryFrom)]
#[try_from(repr)]
pub enum AhQvarZin {
    _520mohm = 0x0,
    _175mohm = 0x1,
    _310mohm = 0x2,
    #[default]
    _75mohm = 0x3,
}

/// Represents the gain options for the AH_QVAR chain.
///
/// # Variants
///
/// - `_05`: Gain of 0.5.
/// - `_1`: Gain of 1.
/// - `_2`: Gain of 2.
/// - `_4`: Gain of 4.
///
/// # Description
///
/// This enum is used to specify the gain setting for the AH_QVAR chain.
#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, TryFrom)]
#[try_from(repr)]
pub enum AhQvarGain {
    _05 = 0x0,
    _1 = 0x1,
    _2 = 0x2,
    #[default]
    _4 = 0x3,
}

/// Represents the threshold options for 4D/6D detection.
///
/// # Variants
///
/// - `_80deg`: 80-degree threshold.
/// - `_70deg`: 70-degree threshold.
/// - `_60deg`: 60-degree threshold.
/// - `_50deg`: 50-degree threshold.
///
/// # Description
///
/// This enum is used to specify the threshold for 4D/6D detection, allowing for various sensitivity levels.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Threshold {
    #[default]
    _80deg = 0x0,
    _70deg = 0x1,
    _60deg = 0x2,
    _50deg = 0x3,
}

/// Represents the mode options for 4D/6D detection.
///
/// # Variants
///
/// - `_6d`: 6D detection mode.
/// - `_4d`: 4D detection mode.
///
/// # Description
///
/// This enum is used to specify the mode for 4D/6D detection, allowing for either 4D or 6D configurations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Mode {
    #[default]
    _6d = 0x0,
    _4d = 0x1,
}

/// Represents the wake duration options for the wakeup interrupt.
///
/// # Variants
///
/// - `_0Odr`: 0 ODR time.
/// - `_1Odr`: 1 ODR time.
/// - `_2Odr`: 2 ODR time.
/// - `_3Odr`: 3 ODR time (when `WU_DUR_EXTENDED` is set).
/// - `_7Odr`: 7 ODR time (when `WU_DUR_EXTENDED` is set).
/// - `_11Odr`: 11 ODR time (when `WU_DUR_EXTENDED` is set).
/// - `_15Odr`: 15 ODR time (when `WU_DUR_EXTENDED` is set).
///
/// # Description
///
/// This enum is used to specify the wake duration for the wakeup interrupt. The wake duration defines
/// the minimum time that the accelerometer data must exceed the configured threshold to generate a
/// wakeup interrupt. The duration is expressed in multiples of the accelerometer output data rate (ODR).
///
/// The duration can be extended by setting the `WU_DUR_EXTENDED` bit in the `WAKE_UP_DUR_EXT` register.
/// When extended, the durations are selectable as 3, 7, 11, or 15 ODR times.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default)]
pub enum WakeDur {
    #[default]
    _0Odr,
    _1Odr,
    _2Odr,
    _3Odr,
    _7Odr,
    _11Odr,
    _15Odr,
}

impl WakeDur {
    /// Retrieves the extended wake duration bit (`WU_DUR_EXTENDED`).
    ///
    /// # Returns
    ///
    /// - `u8`:
    ///   - `0`: Indicates that the wake duration is not extended.
    ///   - `1`: Indicates that the wake duration is extended.
    ///
    /// # Description
    ///
    /// This function determines whether the wake duration is extended by checking the `WU_DUR_EXTENDED` bit.
    /// Extended durations allow for longer wakeup times (3, 7, 11, or 15 ODR times).
    pub fn wake_up_dur_ext(&self) -> u8 {
        match self {
            WakeDur::_0Odr | WakeDur::_1Odr | WakeDur::_2Odr => 0x0,
            WakeDur::_3Odr | WakeDur::_7Odr | WakeDur::_11Odr | WakeDur::_15Odr => 0x1,
        }
    }

    /// Retrieves the wake duration value (`WAKE_DUR`).
    ///
    /// # Returns
    ///
    /// - `u8`: The wake duration value corresponding to the selected duration.
    ///
    /// # Description
    ///
    /// This function retrieves the wake duration value based on the selected duration. The value is used
    /// to configure the `WAKE_DUR` bits in the `WAKE_UP_DUR` register.
    pub fn wake_dur(&self) -> u8 {
        match self {
            WakeDur::_0Odr | WakeDur::_3Odr => 0x00,
            WakeDur::_1Odr | WakeDur::_7Odr => 0x01,
            WakeDur::_2Odr | WakeDur::_11Odr => 0x02,
            WakeDur::_15Odr => 0x03,
        }
    }

    /// Creates a new `WakeDur` instance based on the `WU_DUR_EXTENDED` bit and the `WAKE_DUR` value.
    ///
    /// # Arguments
    ///
    /// - `wup_dur_ext: u8`: The value of the `WU_DUR_EXTENDED` bit (0 or 1).
    /// - `wake_dur: u8`: The value of the `WAKE_DUR` bits.
    ///
    /// # Returns
    ///
    /// - `WakeDur`: The corresponding `WakeDur` variant.
    ///
    /// # Description
    ///
    /// This function creates a new `WakeDur` instance by interpreting the `WU_DUR_EXTENDED` bit and the
    /// `WAKE_DUR` value. It maps the combination of these values to the appropriate wake duration.
    pub fn new(wup_dur_ext: u8, wake_dur: u8) -> Self {
        match wake_dur {
            0x0 => {
                if wup_dur_ext == 1 {
                    WakeDur::_3Odr
                } else {
                    WakeDur::_0Odr
                }
            }
            0x1 => {
                if wup_dur_ext == 1 {
                    WakeDur::_7Odr
                } else {
                    WakeDur::_1Odr
                }
            }
            0x2 => {
                if wup_dur_ext == 1 {
                    WakeDur::_11Odr
                } else {
                    WakeDur::_2Odr
                }
            }
            _ => WakeDur::_15Odr,
        }
    }
}

/// Represents the wake enable options.
///
/// # Variants
///
/// - `SleepOff`: Wakeup function is disabled.
/// - `SleepOn`: Wakeup function is enabled.
///
/// # Description
///
/// This enum is used to specify whether the wakeup function is enabled or disabled.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum WakeEnable {
    #[default]
    SleepOff = 0,
    SleepOn = 1,
}

/// Represents the inactivity ODR options.
///
/// # Variants
///
/// - `NoChange`: No change in ODR.
/// - `_1_6hz`: 1.6 Hz ODR.
/// - `_3hz`: 3 Hz ODR.
/// - `_25hz`: 25 Hz ODR.
///
/// # Description
///
/// This enum is used to specify the inactivity ODR for the wakeup function.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum InactOdr {
    #[default]
    NoChange = 0,
    _1_6hz = 1,
    _3hz = 2,
    _25hz = 3,
}

/// Represents the axis selection for tap detection.
///
/// # Variants
///
/// - `TapNone`: No axis is selected for tap detection.
/// - `TapOnX`: Tap detection is enabled on the X-axis.
/// - `TapOnY`: Tap detection is enabled on the Y-axis.
/// - `TapOnZ`: Tap detection is enabled on the Z-axis.
///
/// # Description
///
/// This enum is used to specify the axis for tap detection, allowing for detection on the X, Y, or Z axis,
/// or disabling tap detection entirely.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Axis {
    #[default]
    TapNone = 0x0,
    TapOnX = 0x1,
    TapOnY = 0x2,
    TapOnZ = 0x3,
}

/// Represents the FIFO operation modes.
///
/// # Variants
///
/// - `BypassMode`: Bypass mode.
/// - `FifoMode`: FIFO mode.
/// - `StreamToFifoMode`: Stream-to-FIFO mo
/// - `BypassToStreamMode`: Bypass-to-stream mode.
/// - `StreamMode`: Stream mode.
/// - `BypassToFifoMode`: Bypass-to-FIFO mode.
/// - `FifoOff`: FIFO is off.
///
/// # Description
///
/// This enum is used to specify the FIFO operation mode, allowing for various configurations such
/// as bypass, FIFO, and stream modes.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoOperation {
    #[default]
    BypassMode = 0x0,
    FifoMode = 0x1,
    StreamToFifoMode = 0x3,
    BypassToStreamMode = 0x4,
    StreamMode = 0x6,
    BypassToFifoMode = 0x7,
    FifoOff = 0x8,
}

/// Represents the storage depth options for the FIFO.
///
/// # Variants
///
/// - `Fifo1x`: 1X storage depth.
/// - `Fifo2x`: 2X storage depth.
///
/// # Description
///
/// This enum is used to specify the storage depth for the FIFO, allowing for 1X or 2X configurations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Store {
    #[default]
    Fifo1x = 0,
    Fifo2x = 1,
}

/// Represents the decimation options for timestamp batching.
///
/// # Variants
///
/// - `Off`: Timestamp batching is off.
/// - `_1`: Decimation of 1.
/// - `_8`: Decimation of 8.
/// - `_32`: Decimation of 32.
///
/// # Description
///
/// This enum is used to specify the decimation for timestamp batching in the FIFO.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DecTs {
    #[default]
    Off = 0x0,
    _1 = 0x1,
    _8 = 0x2,
    _32 = 0x3,
}

/// Represents the accelerometer batch data rate options.
///
/// # Variants
///
/// - `Odr`: Output data rate.
/// - `OdrDiv2`: Output data rate divided by 2.
/// - `OdrDiv4`: Output data rate divided by 4.
/// - `OdrDiv8`: Output data rate divided by 8.
/// - `OdrDiv16`: Output data rate divided by 16.
/// - `OdrDiv32`: Output data rate divided by 32.
/// - `OdrDiv64`: Output data rate divided by 64.
/// - `OdrOff`: Output data rate is off.
///
/// # Description
///
/// This enum is used to specify the accelerometer batch data rate for the FIFO.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum BdrXl {
    #[default]
    Odr = 0x0,
    OdrDiv2 = 0x1,
    OdrDiv4 = 0x2,
    OdrDiv8 = 0x3,
    OdrDiv16 = 0x4,
    OdrDiv32 = 0x5,
    OdrDiv64 = 0x6,
    OdrOff = 0x7,
}

/// Represents the FIFO event types.
///
/// # Variants
///
/// - `FifoEvWtm`: FIFO watermark event.
/// - `FifoEvFull`: FIFO full event.
///
/// # Description
///
/// This enum is used to specify the FIFO event type, allowing for watermark or full event configurations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoEvent {
    #[default]
    Wtm = 0x0,
    Full = 0x1,
}

/// Represents the initialization modes for the device.
///
/// # Variants
///
/// - `SensorOnlyOn`: Activates sensor-only mode without embedded functions.
/// - `Boot`: Initiates the boot procedure.
/// - `Reset`: Performs a software reset.
/// - `SensorEmbFuncOn`: Activates sensor mode with embedded functions.
///
/// # Description
///
/// This enum is used to specify the initialization mode for the device. It is used in functions
/// that configure the bus operating mode based on the initialization value.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Init {
    SensorOnlyOn = 0x00,
    Boot = 0x01,
    Reset = 0x02,
    SensorEmbFuncOn = 0x03,
}

/// Represents the data-ready mode options.
///
/// # Variants
///
/// - `DrdyLatched`: Represents the latched data-ready mode.
/// - `DrdyPulsed`: Represents the pulsed data-ready mode.
///
/// # Description
///
/// This enum is used to specify the data-ready mode for operations involving data-ready signals.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DataReadyMode {
    #[default]
    Latched = 0x0,
    Pulsed = 0x1,
}

/// Represents the output data rate (ODR) options.
///
/// # Variants
///
/// - `Off`: ODR is off.
/// - `_1_6hzUlp`: Ultra-low power mode at 1.6 Hz.
/// - `_3hzUlp`: Ultra-low power mode at 3 Hz.
/// - `_25hzUlp`: Ultra-low power mode at 25 Hz.
/// - `_6hzLp`: Low-power mode at 6 Hz.
/// - `_12_5hzLp`: Low-power mode at 12.5 Hz.
/// - `_25hzLp`: Low-power mode at 25 Hz.
/// - `_50hzLp`: Low-power mode at 50 Hz.
/// - `_100hzLp`: Low-power mode at 100 Hz.
/// - `_200hzLp`: Low-power mode at 200 Hz.
/// - `_400hzLp`: Low-power mode at 400 Hz.
/// - `_800hzLp`: Low-power mode at 800 Hz.
/// - `_6hzHp`: High-performance mode at 6 Hz.
/// - `_12_5hzHp`: High-performance mode at 12.5 Hz.
/// - `_25hzHp`: High-performance mode at 25 Hz.
/// - `_50hzHp`: High-performance mode at 50 Hz.
/// - `_100hzHp`: High-performance mode at 100 Hz.
/// - `_200hzHp`: High-performance mode at 200 Hz.
/// - `_400hzHp`: High-performance mode at 400 Hz.
/// - `_800hzHp`: High-performance mode at 800 Hz.
/// - `OdrTrigPin`: Triggered by pin.
/// - `OdrTrigSw`: Triggered by software.
///
/// # Description
///
/// This enum is used to specify the ODR for sensor operations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Odr {
    #[default]
    Off = 0x00,
    _1_6hzUlp = 0x01,
    _3hzUlp = 0x02,
    _25hzUlp = 0x03,
    _6hzLp = 0x04,
    _12_5hzLp = 0x05,
    _25hzLp = 0x06,
    _50hzLp = 0x07,
    _100hzLp = 0x08,
    _200hzLp = 0x09,
    _400hzLp = 0x0A,
    _800hzLp = 0x0B,
    _6hzHp = 0x14,
    _12_5hzHp = 0x15,
    _25hzHp = 0x16,
    _50hzHp = 0x17,
    _100hzHp = 0x18,
    _200hzHp = 0x19,
    _400hzHp = 0x1A,
    _800hzHp = 0x1B,
    TrigPin = 0x2E,
    TrigSw = 0x2F,
}

impl Odr {
    pub fn new(odr: u8, hp_en: u8) -> Self {
        match odr {
            0x00 => Odr::Off,
            0x01 => Odr::_1_6hzUlp,
            0x02 => Odr::_3hzUlp,
            0x03 => Odr::_25hzUlp,
            0x04 => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_6hzHp
                } else {
                    Odr::_6hzLp
                }
            }
            0x05 => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_12_5hzHp
                } else {
                    Odr::_12_5hzLp
                }
            }
            0x06 => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_25hzHp
                } else {
                    Odr::_25hzLp
                }
            }
            0x07 => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_50hzHp
                } else {
                    Odr::_50hzLp
                }
            }
            0x08 => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_100hzHp
                } else {
                    Odr::_100hzLp
                }
            }
            0x09 => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_200hzHp
                } else {
                    Odr::_200hzLp
                }
            }
            0x0A => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_400hzHp
                } else {
                    Odr::_400hzLp
                }
            }
            0x0B => {
                if hp_en == PROPERTY_ENABLE {
                    Odr::_800hzHp
                } else {
                    Odr::_800hzLp
                }
            }
            0x0E => Odr::TrigPin,
            0x0F => Odr::TrigSw,
            _ => Odr::Off,
        }
    }
}

/// Represents the full-scale (FS) options.
///
/// # Variants
///
/// - `_2g`: Full-scale range of ±2g.
/// - `_4g`: Full-scale range of ±4g.
/// - `_8g`: Full-scale range of ±8g.
/// - `_16g`: Full-scale range of ±16g.
///
/// # Description
///
/// This enum is used to specify the FS for sensor operations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Fs {
    #[default]
    _2g = 0,
    _4g = 1,
    _8g = 2,
    _16g = 3,
}

/// Represents the bandwidth (BW) options.
///
/// # Variants
///
/// - `Div2`: Bandwidth is ODR divided by 2.
/// - `Div4`: Bandwidth is ODR divided by 4.
/// - `Div8`: Bandwidth is ODR divided by 8.
/// - `Div16`: Bandwidth is ODR divided by 16.
///
/// # Description
///
/// This enum is used to specify the bandwidth for sensor operations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Bw {
    #[default]
    OdrDiv2 = 0,
    OdrDiv4 = 1,
    OdrDiv8 = 2,
    OdrDiv16 = 3,
}

/// Represents the self-test modes for the accelerometer.
///
/// # Variants
///
/// - `Disable`: Disables the self-test.
/// - `Positive`: Enables positive self-test.
/// - `Negative`: Enables negative self-test.
///
/// # Description
///
/// This enum is used to specify the self-test mode for the accelerometer, allowing for positive or
/// negative self-test configurations.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum XlSelfTest {
    Disable = 0x0,
    Positive = 0x1,
    Negative = 0x2,
}

/// Represents the interrupt activation level.
///
/// # Variants
///
/// - `ActiveHigh`: Interrupt is active high.
/// - `ActiveLow`: Interrupt is active low.
///
/// # Description
///
/// This enum is used to specify the activation level of the interrupt pin. It is used in functions
/// that configure or retrieve the interrupt activation level.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IntPinPolarity {
    #[default]
    ActiveHigh = 0x0,
    ActiveLow = 0x1,
}

/// Represents the SPI communication mode.
///
/// # Variants
///
/// - `Spi4Wire`: 4-wire SPI mode.
/// - `Spi3Wire`: 3-wire SPI mode.
///
/// # Description
///
/// This enum is used to specify the SPI communication mode for the device. It allows for configuration
/// of either 4-wire or 3-wire SPI communication, depending on the hardware setup and requirements.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SpiMode {
    #[default]
    Spi4Wire = 0x0,
    Spi3Wire = 0x1,
}

/// Represents the tags used in the FIFO sensor data.
///
/// # Variants
///
/// - `FifoEmpty`: Indicates that the FIFO is empty.
/// - `XlTempTag`: Accelerometer and temperature data.
/// - `XlOnly2xTag`: Accelerometer data only, 2x mode.
/// - `TimestampTag`: Timestamp data.
/// - `StepCounterTag`: Step counter data.
/// - `MlcResultTag`: Machine Learning Core (MLC) result data.
/// - `MlcFilterTag`: MLC filter data.
/// - `MlcFeature`: MLC feature data.
/// - `FsmResultTag`: Finite State Machine (FSM) result data.
/// - `XlOnly2xTag2nd`: Accelerometer data only, 2x mode (second instance).
/// - `XlAndQvar`: Accelerometer and AH_QVAR data.
///
/// # Description
///
/// This enum is used to identify the type of data stored in the FIFO. Each tag corresponds to a specific
/// type of data, such as accelerometer, temperature, timestamp, or results from the Machine Learning Core
/// (MLC) or Finite State Machine (FSM). The tags help in interpreting the data retrieved from the FIFO.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoSensorTag {
    #[default]
    FifoEmpty = 0x0,
    XlTempTag = 0x2,
    XlOnly2xTag = 0x3,
    TimestampTag = 0x4,
    StepCounterTag = 0x12,
    MlcResultTag = 0x1A,
    MlcFilterTag = 0x1B,
    MlcFeature = 0x1C,
    FsmResultTag = 0x1D,
    XlOnly2xTag2nd = 0x1E,
    XlAndQvar = 0x1F,
}

/// Represents the Free Fall threshold options.
///
/// # Variants
///
/// - `_156mg`: 156 mg threshold.
/// - `_219mg`: 219 mg threshold.
/// - `_250mg`: 250 mg threshold.
/// - `_312mg`: 312 mg threshold.
/// - `_344mg`: 344 mg threshold.
/// - `_406mg`: 406 mg threshold.
/// - `_469mg`: 469 mg threshold.
/// - `_500mg`: 500 mg threshold.
///
/// # Description
///
/// This enum is used to specify the threshold value for Free Fall detection, allowing for various
/// sensitivity levels.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FfThreshold {
    #[default]
    _156mg = 0x0,
    _219mg = 0x1,
    _250mg = 0x2,
    _312mg = 0x3,
    _344mg = 0x4,
    _406mg = 0x5,
    _469mg = 0x6,
    _500mg = 0x7,
}
