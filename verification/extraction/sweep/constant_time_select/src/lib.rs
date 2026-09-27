// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/util/constant_time/select.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/util/constant_time/select.rs"]
pub mod select;

pub fn ct_select_u8(cond: bool, a: u8, b: u8) -> u8 {
    select::ct_select_u8(cond, a, b)
}

pub fn ct_select_u16(cond: bool, a: u16, b: u16) -> u16 {
    select::ct_select_u16(cond, a, b)
}

pub fn ct_select_u32(cond: bool, a: u32, b: u32) -> u32 {
    select::ct_select_u32(cond, a, b)
}

pub fn ct_select_u64(cond: bool, a: u64, b: u64) -> u64 {
    select::ct_select_u64(cond, a, b)
}

pub fn ct_select_usize(cond: bool, a: usize, b: usize) -> usize {
    select::ct_select_usize(cond, a, b)
}

pub fn ct_select_u64_bit(cond_bit: u64, a: u64, b: u64) -> u64 {
    select::ct_select_u64_bit(cond_bit, a, b)
}

