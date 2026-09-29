// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/types/bar.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub mod memory;

pub fn pcibar_size(this: crate::drivers::pci::types::bar::PciBar) -> u64 {
    this.size()
}

pub fn pcibar_is_memory(this: crate::drivers::pci::types::bar::PciBar) -> bool {
    this.is_memory()
}

pub fn pcibar_is_io(this: crate::drivers::pci::types::bar::PciBar) -> bool {
    this.is_io()
}

pub fn pcibar_is_64bit(this: crate::drivers::pci::types::bar::PciBar) -> bool {
    this.is_64bit()
}

pub fn pcibar_is_prefetchable(this: crate::drivers::pci::types::bar::PciBar) -> bool {
    this.is_prefetchable()
}

pub fn pcibar_is_present(this: crate::drivers::pci::types::bar::PciBar) -> bool {
    this.is_present()
}

