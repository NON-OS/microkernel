// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/hpet/registers.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/hpet/registers.rs"]
pub mod registers;

pub fn timer_config(n: u8) -> u64 {
    registers::timer_config(n)
}

pub fn timer_comparator(n: u8) -> u64 {
    registers::timer_comparator(n)
}

pub fn timer_fsb_route(n: u8) -> u64 {
    registers::timer_fsb_route(n)
}

