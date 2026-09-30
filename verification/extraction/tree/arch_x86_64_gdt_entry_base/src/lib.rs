// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/gdt/entry_base.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn gdtentry_null() -> crate::arch::x86_64::gdt::entry_base::GdtEntry {
    crate::arch::x86_64::gdt::entry_base::GdtEntry::null()
}

pub fn gdtentry_new(base: u32, limit: u32, access: u8, flags: u8) -> crate::arch::x86_64::gdt::entry_base::GdtEntry {
    crate::arch::x86_64::gdt::entry_base::GdtEntry::new(base, limit, access, flags)
}

pub fn gdtentry_is_present(this: crate::arch::x86_64::gdt::entry_base::GdtEntry) -> bool {
    this.is_present()
}

pub fn gdtentry_dpl(this: crate::arch::x86_64::gdt::entry_base::GdtEntry) -> u8 {
    this.dpl()
}

pub fn gdtentry_is_code(this: crate::arch::x86_64::gdt::entry_base::GdtEntry) -> bool {
    this.is_code()
}

pub fn gdtentry_is_long_mode(this: crate::arch::x86_64::gdt::entry_base::GdtEntry) -> bool {
    this.is_long_mode()
}

