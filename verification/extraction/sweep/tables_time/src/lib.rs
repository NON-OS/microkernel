// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/tables/time.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/tables/time.rs"]
pub mod time;

pub fn efitime_is_valid(this: time::EfiTime) -> bool {
    this.is_valid()
}

pub fn efitime_to_unix_timestamp(this: time::EfiTime) -> i64 {
    this.to_unix_timestamp()
}

