use crate::{BusOperation, DelayNs, Error, Iis2dulpx};
use bitfield_struct::bitfield;
use st_mem_bank_macro::adv_register;
use st_mems_bus::EmbAdvFunctions;

/// Represents the register addresses for embedded advanced features page 0.
///
/// These registers are accessible when `PAGE_SEL[3:0]` are set to `0000` in the `PAGE_SEL` register.
/// The content of these registers is loaded when the embedded functions are enabled by setting the
/// `EMB_FUNC_EN` bit to 1 in the `CTRL4` register. The embedded functions must be enabled for these
/// registers to become accessible.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum EmbAdvReg {
    /// Address for the `EMB_ADV_PG0` register.
    EmbAdvPg0 = 0x00,
    /// Address for the `FSM_LC_TIMEOUT_L` register (R/W).
    FsmLcTimeoutL = 0x54,
    /// Address for the `FSM_LC_TIMEOUT_H` register (R/W).
    FsmLcTimeoutH = 0x55,
    /// Address for the `FSM_PROGRAMS` register (R/W).
    FsmPrograms = 0x56,
    /// Address for the `FSM_START_ADD_L` register (R/W).
    FsmStartAddL = 0x58,
    /// Address for the `FSM_START_ADD_H` register (R/W).
    FsmStartAddH = 0x59,
    /// Address for the `PEDO_CMD_REG` register (R/W).
    PedoCmdReg = 0x5D,
    /// Address for the `PEDO_DEB_STEPS_CONF` register (R/W).
    PedoDebStepsConf = 0x5E,
    /// Address for the `PEDO_SC_DELTAT_L` register (R/W).
    PedoScDeltatL = 0xAA,
    /// Address for the `PEDO_SC_DELTAT_H` register (R/W).
    PedoScDeltatH = 0xAB,
    /// Address for the `T_AH_QVAR_SENSITIVITY_L` register (R/W).
    TAhQvarSensitivityL = 0xB6,
    /// Address for the `T_AH_QVAR_SENSITIVITY_H` register (R/W).
    TAhQvarSensitivityH = 0xB7,
    /// Address for the `SMART_POWER_CTRL` register (R/W).
    SmartPowerCtrl = 0xD2,
}

/// FSM long counter timeout low register (R/W).
///
/// The `FSM_LC_TIMEOUT_L` register holds the least significant byte of the long counter timeout value.
/// The `FSM_LC_TIMEOUT_H` register holds the most significant byte of the long counter timeout value.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::FsmLcTimeoutL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct FsmLcTimeout {
    /// FSM long counter timeout value.
    #[bits(16, default = 0)]
    pub fsm_lc_timeout: u16,
}

/// FSM number of programs register (R/W).
///
/// The `FSM_PROGRAMS` register specifies the number of FSM programs configured.
/// The number of programs must be less than or equal to 8.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::FsmPrograms, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmPrograms {
    /// Number of FSM programs configured.
    ///
    /// This field indicates the number of FSM programs that are currently configured.
    #[bits(8)]
    pub fsm_n_prog: u8,
}

/// FSM start address low register (R/W).
///
/// The `FSM_START_ADD_L` register holds the least significant byte of the FSM start address.
/// The `FSM_START_ADD_H` register holds the most significant byte of the FSM start address.
/// The first available address is 0x19C.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::FsmStartAddL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct FsmStartAdd {
    /// FSM start address.
    #[bits(16)]
    pub fsm_start: u16,
}

/// Pedometer command register (R/W).
///
/// The `PEDO_CMD_REG` register is used to configure the pedometer's operational features, such as false-positive rejection and carry count interrupt generation.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::PedoCmdReg, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PedoCmdReg {
    #[bits(2, access = RO)]
    not_used0: u8,

    /// Enables the false-positive rejection feature.
    ///
    /// This bit is active if the MLC_EN bit of EMB_FUNC_EN_B or the MLC_BEFORE_FSM_EN bit in the EMB_FUNC_EN_A register is set to 1.
    #[bits(1)]
    pub fp_rejection_en: u8,

    /// Enables interrupt generation only on count overflow event.
    ///
    /// When set, the pedometer generates an interrupt only when the step count overflows.
    #[bits(1)]
    pub carry_count_en: u8,

    #[bits(4, access = RO, default = 0)]
    not_used1: u8,
}

/// Pedometer debounce steps configuration register (R/W).
///
/// The `PEDO_DEB_STEPS_CONF` register sets the debounce threshold, which is the minimum number of steps required to increment the step counter.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::PedoDebStepsConf, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PedoDebStepsConf {
    /// Debounce threshold for step counting.
    ///
    /// This field sets the minimum number of steps required to increment the step counter.
    #[bits(8, default = 0b00001010)]
    pub deb_step: u8,
}

/// Time period register for step detection on delta time (R/W).
///
/// The `PEDO_SC_DELTAT_L` register holds the least significant byte of the time period value for step detection based on delta time.
/// The `PEDO_SC_DELTAT_H` register holds the most significant byte of the time period value for step detection based on delta time.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::PedoScDeltatL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct PedoScDeltat {
    /// Time period value.
    #[bits(16)]
    pub pd_sc: u16,
}

/// Temperature / analog hub / Qvar sensor sensitivity low register (R/W).
///
/// The `T_AH_QVAR_SENSITIVITY_L` register holds the least significant byte of the sensitivity value for the temperature, analog hub, or Qvar sensor.
/// The `T_AH_QVAR_SENSITIVITY_H` register holds the most significant byte of the sensitivity value for the temperature, analog hub, or Qvar sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::TAhQvarSensitivityL, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct TAhQvarSensitivity {
    /// Sensor sensitivity value.
    #[bits(16, default = 0b1101000100011001)]
    pub t_ah_qvar_s: u16,
}

/// Smart power management configuration register (R/W).
///
/// The `SMART_POWER_CTRL` register configures the smart power management feature, including the duration threshold and the number of consecutive windows for evaluation.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[adv_register(base_address = EmbAdvReg::EmbAdvPg0, address = EmbAdvReg::SmartPowerCtrl, access_type = Iis2dulpx, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SmartPowerCtrl {
    /// Duration threshold for smart power management.
    ///
    /// This field sets the value of the duration threshold for the smart power management feature.
    #[bits(4)]
    pub smart_power_ctrl_dur: u8,

    /// Number of consecutive windows for smart power evaluation.
    ///
    /// This field sets the number of consecutive windows during which the smart power management feature is evaluated.
    #[bits(4)]
    pub smart_power_ctrl_win: u8,
}

/// Represents the configuration settings for the smart power functionality.
///
/// # Fields
///
/// - `enable: u8`: Indicates if the smart power functionality is enabled.
/// - `window: u8`: The window setting for the smart power functionality.
/// - `duration: u8`: The duration setting for the smart power functionality.
///
/// # Description
///
/// This struct encapsulates the configuration settings for the smart power functionality, allowing
/// customization of enable status, window, and duration.
#[derive(Default)]
pub struct SmartPowerCfg {
    pub enable: u8,
    pub window: u8,
    pub duration: u8,
}
