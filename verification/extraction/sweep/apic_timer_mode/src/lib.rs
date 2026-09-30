// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/interrupt/apic/timer_mode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/interrupt/apic/timer_mode.rs"]
pub mod timer_mode;

pub fn divider_to_code(div: u8) -> u32 {
    timer_mode::divider_to_code(div)
}

pub fn calibrate_timer(hz: u32) -> u32 {
    timer_mode::calibrate_timer(hz)
}

