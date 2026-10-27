// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel heap's front for small blocks: which class a layout is served
//! from, the free lists a slab is cut into, and that a churn of small blocks
//! never reaches the first-fit lists past one slab a class at a time; the
//! kernel's use of it by its source (source.rs).

#[path = "../../../../src/memory/heap/types/size_carve.rs"]
mod size_carve;
#[path = "../../../../src/memory/heap/types/size_class.rs"]
mod size_class;
#[path = "../../../../src/memory/heap/types/size_lists.rs"]
mod size_lists;
mod source;

use size_class::{class_of, SizeClasses, CLASSES, CLASS_SIZES, SLAB_ALIGN, SLAB_BYTES};

/// The heap's wrapper around every block: the 32-byte header before the
/// data at 8-byte alignment, and the 8-byte canary after it.
const WRAP: usize = 32 + 8;

/// Slabs, page aligned, standing in for what a first-fit list hands out.
struct Arena {
    mem: Vec<u8>,
    next: usize,
    taken: usize,
}

impl Arena {
    fn new(slabs: usize) -> Self {
        Self { mem: vec![0u8; (slabs + 1) * SLAB_BYTES], next: 0, taken: 0 }
    }

    fn slab(&mut self) -> *mut u8 {
        let base = self.mem.as_mut_ptr() as usize;
        let start = (base + SLAB_ALIGN - 1) / SLAB_ALIGN * SLAB_ALIGN + self.next;
        assert!(start + SLAB_BYTES <= base + self.mem.len(), "arena exhausted");
        self.next += SLAB_BYTES;
        self.taken += 1;
        start as *mut u8
    }
}

/// What raw_alloc does for a small layout, against the arena.
fn take(classes: &mut SizeClasses, arena: &mut Arena, class: usize) -> *mut u8 {
    unsafe { classes.pop(class).unwrap_or_else(|| classes.carve(class, arena.slab())) }
}

mod churn_tests;
mod class_tests;
mod slab_tests;
