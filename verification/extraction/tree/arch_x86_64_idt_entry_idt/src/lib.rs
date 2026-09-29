// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/idt/entry_idt.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn idtentry_empty() -> crate::arch::x86_64::idt::entry_idt::IdtEntry {
    crate::arch::x86_64::idt::entry_idt::IdtEntry::empty()
}

pub fn idtentry_is_present(this: crate::arch::x86_64::idt::entry_idt::IdtEntry) -> bool {
    this.is_present()
}

pub fn idtentry_handler(this: crate::arch::x86_64::idt::entry_idt::IdtEntry) -> u64 {
    this.handler()
}

pub fn idtentry_ist(this: crate::arch::x86_64::idt::entry_idt::IdtEntry) -> u8 {
    this.ist()
}

pub fn idtentry_dpl(this: crate::arch::x86_64::idt::entry_idt::IdtEntry) -> u8 {
    this.dpl()
}

pub fn idtentry_is_trap(this: crate::arch::x86_64::idt::entry_idt::IdtEntry) -> bool {
    this.is_trap()
}

