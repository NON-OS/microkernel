// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/tables/sl_pte.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/tables/sl_pte.rs"]
pub mod sl_pte;

pub fn is_present(entry: u64) -> bool {
    sl_pte::is_present(entry)
}

pub fn entry_address(entry: u64) -> u64 {
    sl_pte::entry_address(entry)
}

pub fn leaf(phys: u64, read: bool, write: bool, snoop: bool) -> u64 {
    sl_pte::leaf(phys, read, write, snoop)
}

pub fn table(phys: u64) -> u64 {
    sl_pte::table(phys)
}

pub fn index_for(addr: u64, level: u8) -> usize {
    sl_pte::index_for(addr, level)
}

pub fn level_span(level: u8) -> u64 {
    sl_pte::level_span(level)
}

pub fn fits_address_width(addr: u64, width_bits: u8) -> bool {
    sl_pte::fits_address_width(addr, width_bits)
}

