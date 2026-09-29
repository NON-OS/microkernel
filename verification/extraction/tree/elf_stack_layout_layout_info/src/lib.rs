// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/elf/stack/layout/layout_info.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod elf;

pub mod memory;

pub fn stacklayout_stack_size(this: crate::elf::stack::layout::layout_info::StackLayout) -> usize {
    this.stack_size()
}

pub fn stacklayout_used_size(this: crate::elf::stack::layout::layout_info::StackLayout) -> usize {
    this.used_size()
}

pub fn stacklayout_available_size(this: crate::elf::stack::layout::layout_info::StackLayout) -> usize {
    this.available_size()
}

