// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/pci/types_msix.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/pci/types_msix.rs"]
pub mod types_msix;

pub fn msixtableentry_new(addr: u64, data: u32) -> types_msix::MsixTableEntry {
    types_msix::MsixTableEntry::new(addr, data)
}

