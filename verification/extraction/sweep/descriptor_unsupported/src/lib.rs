// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/paging/descriptor/unsupported.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/paging/descriptor/unsupported.rs"]
pub mod unsupported;

pub fn leaf(_pa: u64, _flags: u64) -> u64 {
    unsupported::leaf(_pa, _flags)
}

pub fn table(_pa: u64, _user_accessible: bool) -> u64 {
    unsupported::table(_pa, _user_accessible)
}

pub fn is_present(_entry: u64) -> bool {
    unsupported::is_present(_entry)
}

pub fn is_block(_entry: u64) -> bool {
    unsupported::is_block(_entry)
}

pub fn address(entry: u64) -> u64 {
    unsupported::address(entry)
}

pub fn is_writable(_entry: u64) -> bool {
    unsupported::is_writable(_entry)
}

pub fn is_executable(_entry: u64) -> bool {
    unsupported::is_executable(_entry)
}

pub fn is_user(_entry: u64) -> bool {
    unsupported::is_user(_entry)
}

pub fn table_grants_user(_entry: u64) -> bool {
    unsupported::table_grants_user(_entry)
}

