// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/fdt/find/pci/info.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/fdt/find/pci/info.rs"]
pub mod info;

pub fn pcihost_has_io_window(this: info::PciHost) -> bool {
    this.has_io_window()
}

