use crate::{BusOperation, DelayNs, EmbFuncBankState, Error};
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use st_mem_bank_macro::register;

/// Represents the register addresses for embedded functions.
///
/// This enum is used to specify the addresses of various registers related to embedded functions within the device.
/// These registers are accessible when the `EMB_FUNC_REG_ACCESS` bit is set to 1 in the `FUNC_CFG_ACCESS` register
/// and the `EMB_FUNC_EN` bit is set to 1 in the `CTRL4` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum EmbReg {
    /// Address for the `PAGE_SEL` register (R/W).
    PageSel = 0x02,
    /// Address for the `EMB_FUNC_EN_A` register (R/W).
    EmbFuncEnA = 0x04,
    /// Address for the `EMB_FUNC_EN_B` register (R/W).
    EmbFuncEnB = 0x05,
    /// Address for the `EMB_FUNC_EXEC_STATUS` register (R).
    EmbFuncExecStatus = 0x07,
    /// Address for the `PAGE_ADDRESS` register (R/W).
    PageAddress = 0x08,
    /// Address for the `PAGE_VALUE` register (R/W).
    PageValue = 0x09,
    /// Address for the `EMB_FUNC_INT1` register (R/W).
    EmbFuncInt1 = 0x0A,
    /// Address for the `FSM_INT1` register (R/W).
    FsmInt1 = 0x0B,
    /// Address for the `MLC_INT1` register (R/W).
    MlcInt1 = 0x0D,
    /// Address for the `EMB_FUNC_INT2` register (R/W).
    EmbFuncInt2 = 0x0E,
    /// Address for the `FSM_INT2` register (R/W).
    FsmInt2 = 0x0F,
    /// Address for the `MLC_INT2` register (R/W).
    MlcInt2 = 0x11,
    /// Address for the `EMB_FUNC_STATUS` register (R).
    EmbFuncStatus = 0x12,
    /// Address for the `FSM_STATUS` register (R).
    FsmStatus = 0x13,
    /// Address for the `MLC_STATUS` register (R).
    MlcStatus = 0x15,
    /// Address for the `PAGE_RW` register (R/W).
    PageRw = 0x17,
    /// Address for the `EMB_FUNC_FIFO_EN` register (R/W).
    EmbFuncFifoEn = 0x18,
    /// Address for the `FSM_ENABLE` register (R/W).
    FsmEnable = 0x1A,
    /// Address for the `FSM_LONG_COUNTER_L` register (R/W).
    FsmLongCounterL = 0x1C,
    /// Address for the `FSM_LONG_COUNTER_H` register (R/W).
    FsmLongCounterH = 0x1D,
    /// Address for the `INT_ACK_MASK` register (R/W).
    IntAckMask = 0x1F,
    /// Address for the `FSM_OUTS1` register (R).
    FsmOuts1 = 0x20,
    /// Address for the `FSM_OUTS2` register (R).
    FsmOuts2 = 0x21,
    /// Address for the `FSM_OUTS3` register (R).
    FsmOuts3 = 0x22,
    /// Address for the `FSM_OUTS4` register (R).
    FsmOuts4 = 0x23,
    /// Address for the `FSM_OUTS5` register (R).
    FsmOuts5 = 0x24,
    /// Address for the `FSM_OUTS6` register (R).
    FsmOuts6 = 0x25,
    /// Address for the `FSM_OUTS7` register (R).
    FsmOuts7 = 0x26,
    /// Address for the `FSM_OUTS8` register (R).
    FsmOuts8 = 0x27,
    /// Address for the `STEP_COUNTER_L` register (R).
    StepCounterL = 0x28,
    /// Address for the `STEP_COUNTER_H` register (R).
    StepCounterH = 0x29,
    /// Address for the `EMB_FUNC_SRC` register (R/W).
    EmbFuncSrc = 0x2A,
    /// Address for the `EMB_FUNC_INIT_A` register (R/W).
    EmbFuncInitA = 0x2C,
    /// Address for the `EMB_FUNC_INIT_B` register (R/W).
    EmbFuncInitB = 0x2D,
    /// Address for the `MLC1_SRC` register (R).
    Mlc1Src = 0x34,
    /// Address for the `MLC2_SRC` register (R).
    Mlc2Src = 0x35,
    /// Address for the `MLC3_SRC` register (R).
    Mlc3Src = 0x36,
    /// Address for the `MLC4_SRC` register (R).
    Mlc4Src = 0x37,
    /// Address for the `FSM_ODR` register (R/W).
    FsmOdr = 0x39,
    /// Address for the `MLC_ODR` register (R/W).
    MlcOdr = 0x3A,
}

/// Page Selection Register (R/W).
///
/// The `PAGE_SEL` register selects the advanced features dedicated page.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::PageSel, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageSel {
    #[bits(4, access = RO, default = 1)]
    not_used0: u8,

    /// Page Selection.
    ///
    /// Selects the advanced features dedicated page (from 0 to 3). Default value: 0000.
    #[bits(4)]
    pub page_sel: u8,
}

/// Embedded Function Enable A Register (R/W).
///
/// The `EMB_FUNC_EN_A` register enables various embedded functions such as pedometer, tilt, and significant motion detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncEnA, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncEnA {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// Pedometer Enable.
    ///
    /// Enables the pedometer algorithm. Default value: 0.
    #[bits(1)]
    pub pedo_en: u8,

    /// Tilt Enable.
    ///
    /// Enables the tilt calculation. Default value: 0.
    #[bits(1)]
    pub tilt_en: u8,

    /// Significant Motion Enable.
    ///
    /// Enables significant motion detection function. Default value: 0.
    #[bits(1)]
    pub sign_motion_en: u8,

    #[bits(1, access = RO, default = 0)]
    not_used1: u8,

    /// MLC Before FSM Enable.
    ///
    /// Enables machine learning core function to be executed before FSM programs. Default value: 0.
    #[bits(1)]
    pub mlc_before_fsm_en: u8,
}

/// Embedded Function Enable B Register (R/W).
///
/// The `EMB_FUNC_EN_B` register enables the machine learning core and finite state machine functions.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncEnB, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncEnB {
    /// FSM Enable.
    ///
    /// Enables finite state machine (FSM) function. Default value: 0.
    #[bits(1)]
    pub fsm_en: u8,

    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// MLC Enable.
    ///
    /// Enables machine learning core function to be executed after FSM programs. Default value: 0.
    #[bits(1)]
    pub mlc_en: u8,

    #[bits(3, access = RO, default = 0)]
    not_used1: u8,
}

/// Embedded Function Execution Status Register (R).
///
/// The `EMB_FUNC_EXEC_STATUS` register provides the execution status of embedded functions.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncExecStatus, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncExecStatus {
    /// Embedded Function End of Operation.
    ///
    /// 0: Embedded function is running; 1: No embedded function is running.
    #[bits(1, access = RO)]
    pub emb_func_endop: u8,

    /// Embedded Function Execution Overrun.
    ///
    /// 0: Execution within time; 1: Execution exceeds maximum time.
    #[bits(1, access = RO)]
    pub emb_func_exec_ovr: u8,

    #[bits(6, access = RO)]
    not_used0: u8,
}

/// Page Address Register (R/W).
///
/// The `PAGE_ADDRESS` register sets the address of the register to be written/read in the advanced features page.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::PageAddress, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageAddress {
    /// Page Address.
    ///
    /// Sets the address of the register to be written/read in the selected advanced features page.
    #[bits(8)]
    pub page_addr: u8,
}

/// Page Value Register (R/W).
///
/// The `PAGE_VALUE` register is used to write or read the data at the address of the selected advanced features page.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::PageValue, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageValue {
    /// Page Value.
    ///
    /// Used to write or read the data at the address of the selected advanced features page.
    #[bits(8)]
    pub page_value: u8,
}

/// INT1 pin control register for embedded functions (R/W).
///
/// The `EMB_FUNC_INT1` register is used to enable routing of various embedded function events to the INT1 pin.
/// Each bit in this register corresponds to a specific event, and the pin's output supplies the OR combination of the selected signals.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncInt1, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInt1 {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,
    /// Enables routing pedometer step recognition event to INT1.
    ///
    /// When set, the pedometer step recognition event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_step_det: u8,
    /// Enables routing tilt event to INT1.
    ///
    /// When set, the tilt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_tilt: u8,
    /// Enables routing significant motion event to INT1.
    ///
    /// When set, the significant motion event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_sig_mot: u8,
    #[bits(1, access = RO, default = 0)]
    not_used1: u8,
    /// Enables routing FSM long counter timeout interrupt event to INT1.
    ///
    /// When set, the FSM long counter timeout interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm_lc: u8,
}

/// INT1 pin control register for FSM events (R/W).
///
/// The `FSM_INT1` register is used to enable routing of various FSM interrupt events to the INT1 pin.
/// Each bit in this register corresponds to a specific FSM module, and the pin's output supplies the OR combination of the selected signals.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmInt1, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmInt1 {
    /// Enables routing FSM1 interrupt event to INT1.
    ///
    /// When set, the FSM1 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm1: u8,
    /// Enables routing FSM2 interrupt event to INT1.
    ///
    /// When set, the FSM2 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm2: u8,
    /// Enables routing FSM3 interrupt event to INT1.
    ///
    /// When set, the FSM3 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm3: u8,
    /// Enables routing FSM4 interrupt event to INT1.
    ///
    /// When set, the FSM4 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm4: u8,
    /// Enables routing FSM5 interrupt event to INT1.
    ///
    /// When set, the FSM5 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm5: u8,
    /// Enables routing FSM6 interrupt event to INT1.
    ///
    /// When set, the FSM6 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm6: u8,
    /// Enables routing FSM7 interrupt event to INT1.
    ///
    /// When set, the FSM7 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm7: u8,
    /// Enables routing FSM8 interrupt event to INT1.
    ///
    /// When set, the FSM8 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_fsm8: u8,
}

/// INT1 pin control register for MLC events (R/W).
///
/// The `MLC_INT1` register is used to enable routing of various Machine Learning Core (MLC) interrupt events to the INT1 pin.
/// Each bit in this register corresponds to a specific MLC module, and the pin's output supplies the OR combination of the selected signals.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::MlcInt1, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcInt1 {
    /// Enables routing MLC1 interrupt event to INT1.
    ///
    /// When set, the MLC1 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_mlc1: u8,
    /// Enables routing MLC2 interrupt event to INT1.
    ///
    /// When set, the MLC2 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_mlc2: u8,
    /// Enables routing MLC3 interrupt event to INT1.
    ///
    /// When set, the MLC3 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_mlc3: u8,
    /// Enables routing MLC4 interrupt event to INT1.
    ///
    /// When set, the MLC4 interrupt event is routed to the INT1 pin.
    #[bits(1)]
    pub int1_mlc4: u8,
    #[bits(4, access = RO, default = 0)]
    not_used0: u8,
}

/// INT2 pin control register for embedded functions (R/W).
///
/// The `EMB_FUNC_INT2` register is used to enable routing of various embedded function events to the INT2 pin.
/// Each bit in this register corresponds to a specific event, and the pin's output supplies the OR combination of the selected signals.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncInt2, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInt2 {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,
    /// Enables routing pedometer step recognition event to INT2.
    ///
    /// When set, the pedometer step recognition event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_step_det: u8,
    /// Enables routing tilt event to INT2.
    ///
    /// When set, the tilt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_tilt: u8,
    /// Enables routing significant motion event to INT2.
    ///
    /// When set, the significant motion event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_sig_mot: u8,
    #[bits(1, access = RO, default = 0)]
    not_used1: u8,
    /// Enables routing FSM long counter timeout interrupt event to INT2.
    ///
    /// When set, the FSM long counter timeout interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm_lc: u8,
}

/// INT2 pin control register for FSM events (R/W).
///
/// The `FSM_INT2` register is used to enable routing of various FSM interrupt events to the INT2 pin.
/// Each bit in this register corresponds to a specific FSM module, and the pin's output supplies the OR combination of the selected signals.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmInt2, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmInt2 {
    /// Enables routing FSM1 interrupt event to INT2.
    ///
    /// When set, the FSM1 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm1: u8,

    /// Enables routing FSM2 interrupt event to INT2.
    ///
    /// When set, the FSM2 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm2: u8,

    /// Enables routing FSM3 interrupt event to INT2.
    ///
    /// When set, the FSM3 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm3: u8,

    /// Enables routing FSM4 interrupt event to INT2.
    ///
    /// When set, the FSM4 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm4: u8,

    /// Enables routing FSM5 interrupt event to INT2.
    ///
    /// When set, the FSM5 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm5: u8,

    /// Enables routing FSM6 interrupt event to INT2.
    ///
    /// When set, the FSM6 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm6: u8,

    /// Enables routing FSM7 interrupt event to INT2.
    ///
    /// When set, the FSM7 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm7: u8,

    /// Enables routing FSM8 interrupt event to INT2.
    ///
    /// When set, the FSM8 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_fsm8: u8,
}

/// INT2 pin control register for MLC events (R/W).
///
/// The `MLC_INT2` register is used to enable routing of various Machine Learning Core (MLC) interrupt events to the INT2 pin.
/// Each bit in this register corresponds to a specific MLC module, and the pin's output supplies the OR combination of the selected signals.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::MlcInt2, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcInt2 {
    /// Enables routing MLC1 interrupt event to INT2.
    ///
    /// When set, the MLC1 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_mlc1: u8,

    /// Enables routing MLC2 interrupt event to INT2.
    ///
    /// When set, the MLC2 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_mlc2: u8,

    /// Enables routing MLC3 interrupt event to INT2.
    ///
    /// When set, the MLC3 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_mlc3: u8,

    /// Enables routing MLC4 interrupt event to INT2.
    ///
    /// When set, the MLC4 interrupt event is routed to the INT2 pin.
    #[bits(1)]
    pub int2_mlc4: u8,

    #[bits(4, access = RO, default = 0)]
    not_used0: u8,
}

/// Embedded function status register (R).
///
/// The `EMB_FUNC_STATUS` register provides the status of various embedded function events detected by the sensor.
/// It indicates whether specific events, such as step detection, tilt, significant motion, and FSM long counter timeout, have been detected.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncStatus, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncStatus {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// Interrupt status bit for step detection.
    ///
    /// When set, a step detection event has been detected.
    #[bits(1, access = RO)]
    pub is_step_det: u8,

    /// Interrupt status bit for tilt detection.
    ///
    /// When set, a tilt event has been detected.
    #[bits(1, access = RO)]
    pub is_tilt: u8,

    /// Interrupt status bit for significant motion detection.
    ///
    /// When set, a significant motion event has been detected.
    #[bits(1, access = RO)]
    pub is_sigmot: u8,

    /// Reserved bit, not used.
    ///
    /// This field consists of 1 bit reserved for future use or alignment purposes. It is read-only.
    #[bits(1, access = RO, default = 0)]
    not_used1: u8,

    #[bits(1, access = RO)]
    pub is_fsm_lc: u8,
}

/// Finite State Machine status register (R).
///
/// The `FSM_STATUS` register provides the interrupt status for various Finite State Machine (FSM) events.
/// Each bit in this register corresponds to a specific FSM module, indicating whether an interrupt event has been detected.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmStatus, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmStatus {
    /// Interrupt status bit for FSM1 interrupt event.
    ///
    /// When set, an interrupt event for FSM1 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm1: u8,

    /// Interrupt status bit for FSM2 interrupt event.
    ///
    /// When set, an interrupt event for FSM2 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm2: u8,

    /// Interrupt status bit for FSM3 interrupt event.
    ///
    /// When set, an interrupt event for FSM3 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm3: u8,

    /// Interrupt status bit for FSM4 interrupt event.
    ///
    /// When set, an interrupt event for FSM4 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm4: u8,

    /// Interrupt status bit for FSM5 interrupt event.
    ///
    /// When set, an interrupt event for FSM5 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm5: u8,

    /// Interrupt status bit for FSM6 interrupt event.
    ///
    /// When set, an interrupt event for FSM6 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm6: u8,

    /// Interrupt status bit for FSM7 interrupt event.
    ///
    /// When set, an interrupt event for FSM7 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm7: u8,

    /// Interrupt status bit for FSM8 interrupt event.
    ///
    /// When set, an interrupt event for FSM8 has been detected.
    #[bits(1, access = RO)]
    pub is_fsm8: u8,
}

/// Machine Learning Core status register (R).
///
/// The `MLC_STATUS` register provides the interrupt status for various Machine Learning Core (MLC) events.
/// Each bit in this register corresponds to a specific MLC module, indicating whether an interrupt event has been detected.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::MlcStatus, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcStatus {
    /// Interrupt status bit for MLC1 interrupt event.
    ///
    /// When set, an interrupt event for MLC1 has been detected.
    #[bits(1, access = RO)]
    pub is_mlc1: u8,

    /// Interrupt status bit for MLC2 interrupt event.
    ///
    /// When set, an interrupt event for MLC2 has been detected.
    #[bits(1, access = RO)]
    pub is_mlc2: u8,

    /// Interrupt status bit for MLC3 interrupt event.
    ///
    /// When set, an interrupt event for MLC3 has been detected.
    #[bits(1, access = RO)]
    pub is_mlc3: u8,

    /// Interrupt status bit for MLC4 interrupt event.
    ///
    /// When set, an interrupt event for MLC4 has been detected.
    #[bits(1, access = RO)]
    pub is_mlc4: u8,

    #[bits(4, access = RO, default = 0)]
    not_used0: u8,
}

/// Enable read and write mode of advanced features dedicated page (R/W).
///
/// The `PAGE_RW` register is used to enable read and write operations on the advanced features dedicated page.
/// It also configures the latched interrupt mode for embedded functions.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::PageRw, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageRw {
    #[bits(5, access = RO, default = 0)]
    not_used0: u8,

    /// Enable reads from the selected advanced features dedicated page.
    ///
    /// Default value: 0 (1: enable; 0: disable)
    #[bits(1)]
    pub page_read: u8,

    /// Enable writes to the selected advanced features dedicated page.
    ///
    /// Default value: 0 (1: enable; 0: disable)
    #[bits(1)]
    pub page_write: u8,

    /// Latched interrupt mode for embedded functions.
    ///
    /// Default value: 0 (0: not latched; 1: latched)
    #[bits(1)]
    pub emb_func_lir: u8,
}

/// Embedded functions FIFO configuration register (R/W).
///
/// The `EMB_FUNC_FIFO_EN` register configures the FIFO for batching results from various embedded functions.
/// It enables batching for step counter, MLC, and FSM results in the FIFO buffer.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncFifoEn, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncFifoEn {
    /// Enables batching step counter values in the FIFO buffer.
    ///
    /// Default value: 0 (0: disabled; 1: enabled)
    #[bits(1)]
    pub step_counter_fifo_en: u8,

    /// Enables batching machine learning core results in the FIFO buffer.
    ///
    /// Default value: 0 (0: disabled; 1: enabled)
    #[bits(1)]
    pub mlc_fifo_en: u8,

    /// Enables batching machine learning core filters and features in the FIFO buffer.
    ///
    /// Default value: 0 (0: disabled; 1: enabled)
    #[bits(1)]
    pub mlc_filter_feature_fifo_en: u8,

    /// Enables batching finite state machine results in the FIFO buffer.
    ///
    /// Default value: 0 (0: disabled; 1: enabled)
    #[bits(1)]
    pub fsm_fifo_en: u8,

    #[bits(4, access = RO, default = 0)]
    not_used0: u8,
}

/// Enable FSM register (R/W).
///
/// The `FSM_ENABLE` register is used to enable various Finite State Machine (FSM) modules.
/// Each bit in this register corresponds to a specific FSM module, enabling or disabling it.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmEnable, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmEnable {
    /// Enables FSM1.
    ///
    /// Default value: 0 (0: FSM1 disabled; 1: FSM1 enabled)
    #[bits(1)]
    pub fsm1_en: u8,

    /// Enables FSM2.
    ///
    /// Default value: 0 (0: FSM2 disabled; 1: FSM2 enabled)
    #[bits(1)]
    pub fsm2_en: u8,

    /// Enables FSM3.
    ///
    /// Default value: 0 (0: FSM3 disabled; 1: FSM3 enabled)
    #[bits(1)]
    pub fsm3_en: u8,

    /// Enables FSM4.
    ///
    /// Default value: 0 (0: FSM4 disabled; 1: FSM4 enabled)
    #[bits(1)]
    pub fsm4_en: u8,

    /// Enables FSM5.
    ///
    /// Default value: 0 (0: FSM5 disabled; 1: FSM5 enabled)
    #[bits(1)]
    pub fsm5_en: u8,

    /// Enables FSM6.
    ///
    /// Default value: 0 (0: FSM6 disabled; 1: FSM6 enabled)
    #[bits(1)]
    pub fsm6_en: u8,

    /// Enables FSM7.
    ///
    /// Default value: 0 (0: FSM7 disabled; 1: FSM7 enabled)
    #[bits(1)]
    pub fsm7_en: u8,

    /// Enables FSM8.
    ///
    /// Default value: 0 (0: FSM8 disabled; 1: FSM8 enabled)
    #[bits(1)]
    pub fsm8_en: u8,
}

/// FSM long counter status register (R/W).
///
/// The `FSM_LONG_COUNTER_L` and `FSM_LONG_COUNTER_H` registers hold the long counter value in a 16-bit format.
/// These registers are used to track the current value of the FSM long counter.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmLongCounterL, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct FsmLongCounter {
    /// Long counter current value.
    #[bits(16, default = 0)]
    pub fsm_lc: u16,
}

/// Reset status register (R/W).
///
/// The `INT_ACK_MASK` register is used to configure the reset behavior of status bits in latched mode.
/// Each bit in this register corresponds to a specific status bit, determining whether it is reset upon reading.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::IntAckMask, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IntAckMask {
    /// Mask for interrupt acknowledgment bit 0.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask0: u8,

    /// Mask for interrupt acknowledgment bit 1.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask1: u8,

    /// Mask for interrupt acknowledgment bit 2.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask2: u8,

    /// Mask for interrupt acknowledgment bit 3.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask3: u8,

    /// Mask for interrupt acknowledgment bit 4.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask4: u8,

    /// Mask for interrupt acknowledgment bit 5.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask5: u8,

    /// Mask for interrupt acknowledgment bit 6.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask6: u8,

    /// Mask for interrupt acknowledgment bit 7.
    ///
    /// Default value: 0 (0: reset; 1: not reset)
    #[bits(1)]
    pub iack_mask7: u8,
}

/// FSM1-8 output register (R).
///
/// The `FSM_OUTS1-8` register provides the output status for FSM events detected on various axes and vectors.
/// Each bit indicates whether a positive or negative event has been detected.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmOuts1, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts {
    /// FSM output: negative event detected on the vector.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub n_v: u8,

    /// FSM output: positive event detected on the vector.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub p_v: u8,

    /// FSM output: negative event detected on the Z-axis.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub n_z: u8,

    /// FSM output: positive event detected on the Z-axis.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub p_z: u8,

    /// FSM output: negative event detected on the Y-axis.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub n_y: u8,

    /// FSM output: positive event detected on the Y-axis.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub p_y: u8,

    /// FSM output: negative event detected on the X-axis.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub n_x: u8,

    /// FSM output: positive event detected on the X-axis.
    ///
    /// (0: event not detected; 1: event detected)
    #[bits(1, access = RO)]
    pub p_x: u8,
}

/// Step counter output register (R).
///
/// The `STEP_COUNTER_L` register holds the least significant byte of the step counter value.
/// This register is used in conjunction with `STEP_COUNTER_H` to provide the full step count.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::StepCounterL, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct StepCounter {
    /// Step counter output.
    #[bits(16, access = RO)]
    pub step: u16,
}

/// Embedded function source register (R/W).
///
/// The `EMB_FUNC_SRC` register provides the status and control for various embedded functions related to step counting.
/// It includes flags for step detection, overflow, and reset operations.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncSrc, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncSrc {
    #[bits(2, access = RO, default = 0)]
    not_used0: u8,

    /// This bit is equal to 1 when the step count is increased.
    ///
    /// Read-only bit.
    #[bits(1, access = RO)]
    pub stepcounter_bit_set: u8,

    /// Step counter overflow status.
    ///
    /// Read-only bit. (0: step counter value < 2^16; 1: step counter value reached 2^16)
    #[bits(1, access = RO)]
    pub step_overflow: u8,

    /// Pedometer step recognition on delta time status.
    ///
    /// Read-only bit. (0: no step recognized during delta time; 1: at least one step recognized during delta time)
    #[bits(1, access = RO)]
    pub step_count_delta_ia: u8,

    /// Step detector event detection status.
    ///
    /// Read-only bit. (0: step detection event not detected; 1: step detection event detected)
    #[bits(1, access = RO)]
    pub step_detected: u8,

    #[bits(1, access = RO, default = 0)]
    not_used1: u8,

    /// Reset pedometer step counter.
    ///
    /// Read/write bit. (0: disabled; 1: enabled)
    #[bits(1)]
    pub pedo_rst_step: u8,
}

/// Embedded functions initialization register (R/W).
///
/// The `EMB_FUNC_INIT_A` register is used to request initialization for various embedded functions such as step detection and tilt.
/// It also allows configuring the execution order of the MLC and FSM.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncInitA, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInitA {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// Pedometer step counter/detector algorithm initialization request.
    ///
    /// Default value: 0
    #[bits(1)]
    pub step_det_init: u8,

    /// Tilt algorithm initialization request.
    ///
    /// Default value: 0
    #[bits(1)]
    pub tilt_init: u8,

    /// Significant motion detection algorithm initialization request.
    ///
    /// Default value: 0
    #[bits(1)]
    pub sig_mot_init: u8,

    #[bits(1, access = RO, default = 0)]
    not_used1: u8,

    /// Machine learning core initialization request (MLC executed before FSM).
    ///
    /// Default value: 0
    #[bits(1)]
    pub mlc_before_fsm_init: u8,
}

/// Embedded functions initialization register (R/W).
///
/// The `EMB_FUNC_INIT_B` register is used to request initialization for the FSM and MLC.
/// It configures the initialization sequence for these embedded functions.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::EmbFuncInitB, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInitB {
    /// FSM initialization request.
    ///
    /// Default value: 0
    #[bits(1)]
    pub fsm_init: u8,

    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// Machine learning core initialization request (MLC executed after FSM).
    ///
    /// Default value: 0
    #[bits(1)]
    pub mlc_init: u8,

    #[bits(3, access = RO, default = 0)]
    not_used1: u8,
}

/// Machine learning core source register (R).
///
/// The `MLC1-4_SRC` register provides the output value of the MLC decision tree.
/// This register is read-only and reflects the current state of the MLC output.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::Mlc1Src, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcSrc {
    /// Output value of MLC decision tree.
    #[bits(8, access = RO)]
    pub mlc_src: u8,
}

/// Finite State Machine output data rate configuration register (R/W).
///
/// The `FSM_ODR` register configures the output data rate for the Finite State Machine.
/// It allows setting the ODR to various frequencies, providing flexibility in data processing.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::FsmOdr, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOdr {
    #[bits(3, access = RO, default = 0)]
    not_used0: u8,

    /// Finite state machine ODR configuration.
    ///
    /// (000: 12.5 Hz; 001: 25 Hz (default); 010: 50 Hz; 011: 100 Hz; 100: 200 Hz; 101: 400 Hz; 110: 800 Hz)
    #[bits(3, default = 0b001)]
    pub fsm_odr: u8,

    #[bits(2, access = RO, default = 0b01)]
    not_used1: u8,
}

/// Machine Learning Core output data rate configuration register (R/W).
///
/// The `MLC_ODR` register configures the output data rate for the Machine Learning Core.
/// It allows setting the ODR to various frequencies, optimizing the performance of machine learning tasks.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = EmbReg::MlcOdr, access_type = EmbFuncBankState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcOdr {
    #[bits(4, access = RO, default = 0b0001)]
    not_used0: u8,

    /// Machine learning core ODR configuration.
    ///
    /// (000: 12.5 Hz; 001: 25 Hz (default); 010: 50 Hz; 011: 100 Hz; 100: 200 Hz)
    #[bits(3, default = 0b001)]
    pub mlc_odr: u8,

    #[bits(1, access = RO, default = 0)]
    not_used1: u8,
}

/// Represents the status of embedded functions.
///
/// # Fields
///
/// - `is_step_det: u8`: Indicates if a step has been detected.
/// - `is_tilt: u8`: Indicates if a tilt has been detected.
/// - `is_sigmot: u8`: Indicates if significant motion has been detected.
///
/// # Description
///
/// This struct provides the status of embedded functions, offering insights into step detection,
/// tilt detection, and significant motion detection.
pub struct EmbeddedStatus {
    pub is_step_det: u8,
    pub is_tilt: u8,
    pub is_sigmot: u8,
}

/// Represents the embedded function interrupt signals routing configuration.
///
/// # Fields
///
/// - `step_det: u8`: Step detection interrupt.
/// - `tilt: u8`: Tilt detection interrupt.
/// - `sig_mot: u8`: Significant motion detection interrupt.
/// - `fsm_lc: u8`: Finite state machine logic control interrupt.
///
/// # Description
///
/// This struct encapsulates the embedded function interrupt signals routing configuration, allowing
/// customization of various embedded function interrupt sources.
#[derive(Default)]
pub struct EmbPinIntRoute {
    pub step_det: u8,
    pub tilt: u8,
    pub sig_mot: u8,
    pub fsm_lc: u8,
}

/// Represents the configuration settings for the step counter mode.
///
/// # Fields
///
/// - `false_step_rej: u8`: Indicates if false step rejection is enabled.
/// - `step_counter_enable: u8`: Indicates if the step counter is enabled.
/// - `step_counter_in_fifo: u8`: Indicates if the step counter is included in the FIFO.
///
/// # Description
///
/// This struct encapsulates the configuration settings for the step counter mode, allowing customization
/// of false step rejection, step counter enable, and FIFO inclusion.
#[derive(Default)]
pub struct StpcntMode {
    pub false_step_rej: u8,
    pub step_counter_enable: u8,
    pub step_counter_in_fifo: u8,
}

/// Represents the embedded interrupt configuration modes.
///
/// # Variants
///
/// - `IntLevel`: Embedded interrupts are level-sensitive.
/// - `IntLatched`: Embedded interrupts are latched.
///
/// # Description
///
/// This enum is used to specify the embedded interrupt configuration mode, allowing for level-sensitive
/// or latched embedded interrupt settings.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default)]
pub enum EmbeddedIntConfig {
    #[default]
    Level = 0x0,
    Latched = 0x1,
}

/// Represents the FSM output data rate (ODR) options.
///
/// # Variants
///
/// - `_12_5hz`: 12.5 Hz.
/// - `_25hz`: 25 Hz.
/// - `_50hz`: 50 Hz.
/// - `_100hz`: 100 Hz.
/// - `_200hz`: 200 Hz.
/// - `_400hz`: 400 Hz.
/// - `_800hz`: 800 Hz.
///
/// # Description
///
/// This enum is used to specify the FSM output data rate (ODR) for the FSM configuration.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FsmValOdr {
    _12_5hz = 0,
    #[default]
    _25hz = 1,
    _50hz = 2,
    _100hz = 3,
    _200hz = 4,
    _400hz = 5,
    _800hz = 6,
}

/// Represents the Machine Learning Core (MLC) mode.
///
/// # Variants
///
/// - `MlcOff`: MLC is disabled.
/// - `MlcOn`: MLC is enabled.
/// - `MlcOnBeforeFsm`: MLC is enabled before the FSM.
///
/// # Description
///
/// This enum is used to specify the mode of the Machine Learning Core (MLC).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum MlcMode {
    Off = 0,
    On = 1,
    OnBeforeFsm = 2,
}

/// Represents the Machine Learning Core (MLC) output data rate (ODR) options.
///
/// # Variants
///
/// - `_12_5hz`: 12.5 Hz.
/// - `_25hz`: 25 Hz.
/// - `_50hz`: 50 Hz.
/// - `_100hz`: 100 Hz.
/// - `_200hz`: 200 Hz.
///
/// # Description
///
/// This enum is used to specify the output data rate (ODR) for the Machine Learning Core (MLC).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum MlcOdrVal {
    _12_5hz = 0,
    #[default]
    _25hz = 1,
    _50hz = 2,
    _100hz = 3,
    _200hz = 4,
}
