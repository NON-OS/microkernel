// NONOS Operating System (AGPL-3.0-or-later)
// The kernel's alignment helpers, included from the tree so the ordering
// property is proven about the code that computes real addresses. Each module
// defines its own align_up and align_down; they are mirrored together because
// the interesting statements are about the differences between them.

#[path = "../../../../../src/memory/boot_memory/manager/helpers.rs"]
mod boot_memory_helpers;

#[path = "../../../../../src/memory/buddy_alloc/allocator/utils.rs"]
mod buddy_utils;

pub fn boot_align_up(value: u64, align: u64) -> u64 {
    boot_memory_helpers::align_up(value, align)
}

pub fn boot_align_down(value: u64, align: u64) -> u64 {
    boot_memory_helpers::align_down(value, align)
}

pub fn buddy_align_up(value: usize, align: usize) -> usize {
    buddy_utils::align_up(value, align)
}
