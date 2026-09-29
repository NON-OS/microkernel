// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/layout/manager/stack_slots.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn stack_slot_offset(slot: usize) -> u64 {
    crate::memory::layout::manager::stack_slots::stack_slot_offset(slot)
}

