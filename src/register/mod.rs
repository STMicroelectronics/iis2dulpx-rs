pub mod advanced;
pub mod embedded;
pub mod main;

use super::{BusOperation, DelayNs, Error, Iis2dulpx, MemBankFunctions, only_async, only_sync};

use derive_more::TryFrom;
use st_mem_bank_macro::mem_bank;

/// Represents the memory bank options.
///
/// # Variants
///
/// - `MainMemBank`: Represents the main memory bank.
/// - `EmbedFuncMemBank`: Represents the embedded function memory bank.
///
/// # Description
///
/// This enum is used to specify the memory bank for operations involving register access. It provides
/// options for switching between the main memory bank and the embedded function memory bank.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom, Debug)]
#[try_from(repr)]
#[mem_bank(Iis2dulpx, generics = 2)]
pub enum MemBank {
    #[default]
    #[main]
    MainMemBank = 0x0,
    #[state(EmbBank, fn_name = "operate_over_emb")]
    EmbedFuncMemBank = 0x1,
}
