// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/layout/types/stack.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/layout/types/stack.rs"]
pub mod stack;

pub fn stackregion_new(base: u64, size: usize, guard_size: usize) -> stack::StackRegion {
    stack::StackRegion::new(base, size, guard_size)
}

pub fn stackregion_per_cpu(base: u64, size: usize, guard_size: usize, cpu_id: u32) -> stack::StackRegion {
    stack::StackRegion::per_cpu(base, size, guard_size, cpu_id)
}

pub fn stackregion_total_size(this: stack::StackRegion) -> usize {
    this.total_size()
}

pub fn stackregion_stack_top(this: stack::StackRegion) -> u64 {
    this.stack_top()
}

