// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/amd_vi/pte.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/amd_vi/pte.rs"]
pub mod pte;

pub fn directory(table_phys: u64, level: u8) -> u64 {
    pte::directory(table_phys, level)
}

pub fn leaf(phys: u64, read: bool, write: bool) -> u64 {
    pte::leaf(phys, read, write)
}

pub fn is_present(entry: u64) -> bool {
    pte::is_present(entry)
}

pub fn next_level(entry: u64) -> u8 {
    pte::next_level(entry)
}

pub fn address(entry: u64) -> u64 {
    pte::address(entry)
}

pub fn writable(entry: u64) -> bool {
    pte::writable(entry)
}

pub fn index(iova: u64, level: u8) -> usize {
    pte::index(iova, level)
}

pub fn reach(levels: u8) -> u64 {
    pte::reach(levels)
}

