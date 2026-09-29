// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/types/header.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn headertype_is_multifunction(raw: u8) -> bool {
    crate::drivers::pci::types::header::HeaderType::is_multifunction(raw)
}

