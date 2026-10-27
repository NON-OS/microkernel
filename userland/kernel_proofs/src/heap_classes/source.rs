// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's use of the size classes, by its source.

const ROUTE: &str = include_str!("../../../../src/memory/heap/types/region_alloc.rs");
const SMALL: &str = include_str!("../../../../src/memory/heap/types/small_alloc.rs");
const ALLOC: &str = include_str!("../../../../src/memory/heap/types/alloc_impl.rs");
const DEALLOC: &str = include_str!("../../../../src/memory/heap/types/dealloc_impl.rs");

fn body<'a>(src: &'a str, from: &str) -> &'a str {
    let at = src.find(from).unwrap_or_else(|| panic!("{from}"));
    let rest = &src[at..];
    &rest[..rest[1..].find("\n    pub").map_or(rest.len(), |n| n + 1)]
}

#[test]
fn alloc_and_free_choose_the_class_from_the_same_layout() {
    // The wrapped layout both sides pass is the one the class is chosen from.
    assert!(ALLOC.contains("allocator.raw_alloc(adjusted_layout)"));
    assert!(DEALLOC.contains("allocator.raw_dealloc(raw_ptr, adjusted_layout)"));
    let alloc = body(ROUTE, "pub(super) unsafe fn raw_alloc(");
    let free = body(ROUTE, "pub(super) unsafe fn raw_dealloc(");
    for side in [alloc, free] {
        assert!(side.contains("class_of(layout.size(), layout.align())"), "{side}");
    }
    let class =
        free.find("self.small_dealloc(class, ptr)").expect("a small free goes to its class");
    let region = free.find("if self.in_second(ptr)").expect("any other to its region");
    assert!(class < region, "the class is asked first");
    assert!(alloc.contains("None => self.list_alloc(layout)"));
}

#[test]
fn a_slab_is_taken_from_the_lists_with_no_class_lock_held() {
    let pop = SMALL.find("self.small.lock().pop(class)").expect("the list first");
    let slab = SMALL.find("self.list_alloc(slab_layout)").expect("then a slab");
    let carve = SMALL.find("self.small.lock().carve(class, slab)").expect("then the cut");
    assert!(pop < slab && slab < carve);
    // Each `self.small.lock()` is a temporary: no guard is bound across the slab.
    assert!(!SMALL.contains("let mut small = self.small.lock()"));
}

#[test]
fn the_free_bytes_the_floor_reads_count_the_class_lists() {
    let free = body(ROUTE, "pub fn free_bytes(");
    assert!(free.contains("self.small.lock().free_bytes()"));
}
