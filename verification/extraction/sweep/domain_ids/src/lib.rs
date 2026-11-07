// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/drivers/pci/vmd/domain/ids.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/drivers/pci/vmd/domain/ids.rs"]
pub mod ids;

pub fn is_intel_vmd(vendor: u16, device: u16) -> bool {
    ids::is_intel_vmd(vendor, device)
}

