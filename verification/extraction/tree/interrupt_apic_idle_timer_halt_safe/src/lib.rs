// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/interrupt/apic/idle_timer/halt_safe.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn halt_safe() -> bool {
    crate::arch::x86_64::interrupt::apic::idle_timer::halt_safe::halt_safe()
}

