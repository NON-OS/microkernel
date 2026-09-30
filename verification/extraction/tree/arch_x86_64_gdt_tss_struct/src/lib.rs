// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/gdt/tss_struct.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn tss_new() -> crate::arch::x86_64::gdt::tss_struct::Tss {
    crate::arch::x86_64::gdt::tss_struct::Tss::new()
}

pub fn tss_rsp0(this: crate::arch::x86_64::gdt::tss_struct::Tss) -> u64 {
    this.rsp0()
}

