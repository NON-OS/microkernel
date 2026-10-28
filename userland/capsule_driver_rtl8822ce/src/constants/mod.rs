// NONOS Operating System (AGPL-3.0-or-later)

//! The PCI ids this driver takes, and the MAC registers bring-up reads back,
//! which are the RTL8821CE's: the rtw88 MAC map is the same on every 11ac chip.

mod pci;
#[path = "../../../capsule_driver_rtl8821ce/src/constants/regs.rs"]
pub mod regs;

pub use pci::{PCI_VENDOR_REALTEK, RTL8822CE_DEVICE_IDS};
