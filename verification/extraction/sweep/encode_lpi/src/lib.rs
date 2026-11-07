// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/gic/its/encode/lpi.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/gic/its/encode/lpi.rs"]
pub mod lpi;

pub fn config(enabled: bool) -> u8 {
    lpi::config(enabled)
}

pub fn propbaser(phys: u64, id_bits: u32, coherent: bool) -> u64 {
    lpi::propbaser(phys, id_bits, coherent)
}

pub fn pendbaser(phys: u64, coherent: bool) -> u64 {
    lpi::pendbaser(phys, coherent)
}

pub fn prop_bytes(id_bits: u32) -> u64 {
    lpi::prop_bytes(id_bits)
}

pub fn pend_bytes(id_bits: u32) -> u64 {
    lpi::pend_bytes(id_bits)
}

