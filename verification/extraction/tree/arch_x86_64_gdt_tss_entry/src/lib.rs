// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/gdt/tss_entry.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn tssentry_empty() -> crate::arch::x86_64::gdt::tss_entry::TssEntry {
    crate::arch::x86_64::gdt::tss_entry::TssEntry::empty()
}

pub fn tssentry_new(base: u64, limit: u32) -> crate::arch::x86_64::gdt::tss_entry::TssEntry {
    crate::arch::x86_64::gdt::tss_entry::TssEntry::new(base, limit)
}

pub fn tssentry_base(this: crate::arch::x86_64::gdt::tss_entry::TssEntry) -> u64 {
    this.base()
}

pub fn tssentry_is_busy(this: crate::arch::x86_64::gdt::tss_entry::TssEntry) -> bool {
    this.is_busy()
}

