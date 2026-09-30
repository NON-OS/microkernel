// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/gdt/entry_presets.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn gdtentry_kernel_code_64() -> crate::arch::x86_64::gdt::entry_base::GdtEntry {
    crate::arch::x86_64::gdt::entry_base::GdtEntry::kernel_code_64()
}

pub fn gdtentry_kernel_data() -> crate::arch::x86_64::gdt::entry_base::GdtEntry {
    crate::arch::x86_64::gdt::entry_base::GdtEntry::kernel_data()
}

pub fn gdtentry_user_code_64() -> crate::arch::x86_64::gdt::entry_base::GdtEntry {
    crate::arch::x86_64::gdt::entry_base::GdtEntry::user_code_64()
}

pub fn gdtentry_user_data() -> crate::arch::x86_64::gdt::entry_base::GdtEntry {
    crate::arch::x86_64::gdt::entry_base::GdtEntry::user_data()
}

