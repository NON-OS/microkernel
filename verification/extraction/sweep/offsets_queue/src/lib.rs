// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/offsets/queue.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/offsets/queue.rs"]
pub mod queue;

pub fn iqa_value(base_phys: u64) -> u64 {
    queue::iqa_value(base_phys)
}

pub fn queue_offset(index: u16) -> u64 {
    queue::queue_offset(index)
}

pub fn queue_index(register: u64) -> u16 {
    queue::queue_index(register)
}

pub fn queue_next(index: u16) -> u16 {
    queue::queue_next(index)
}

pub fn queue_free(head: u16, tail: u16) -> u16 {
    queue::queue_free(head, tail)
}

