// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for the kernel's alignment helpers.
//!
//! Each module below defines its own `align_up` and `align_down`. They are
//! mirrored here side by side so the differences between them can be stated as
//! theorems rather than noticed by a reader. The constants each file imports are
//! supplied locally, because only the alignment arithmetic is under test.

#[path = "../../../../src/memory/boot_memory/manager/helpers.rs"]
pub mod boot_memory;

#[path = "../../../../src/memory/buddy_alloc/allocator/utils.rs"]
pub mod buddy_alloc;

pub mod layout_outer;
pub mod phys_outer;

pub fn boot_align_up(value: u64, align: u64) -> u64 {
    boot_memory::align_up(value, align)
}

pub fn boot_align_down(value: u64, align: u64) -> u64 {
    boot_memory::align_down(value, align)
}

pub fn buddy_align_up(value: usize, align: usize) -> usize {
    buddy_alloc::align_up(value, align)
}

pub fn layout_align_up(x: u64, a: u64) -> u64 {
    layout_outer::manager::align::align_up(x, a)
}

pub fn layout_align_down(x: u64, a: u64) -> u64 {
    layout_outer::manager::align::align_down(x, a)
}

pub fn phys_align_up(value: u64, align: u64) -> u64 {
    phys_outer::helpers::align_up(value, align)
}

pub fn phys_align_down(value: u64, align: u64) -> u64 {
    phys_outer::helpers::align_down(value, align)
}
