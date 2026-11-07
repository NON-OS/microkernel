// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/pat/value.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/pat/value.rs"]
pub mod value;

pub fn with_wc(pat: u64) -> u64 {
    value::with_wc(pat)
}

pub fn index(pat_bit: bool, pcd: bool, pwt: bool) -> u32 {
    value::index(pat_bit, pcd, pwt)
}

pub fn entry(pat: u64, i: u32) -> u8 {
    value::entry(pat, i)
}

