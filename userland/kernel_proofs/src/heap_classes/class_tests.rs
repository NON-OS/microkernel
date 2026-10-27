// NONOS Operating System (AGPL-3.0-or-later)
//! Which class a layout is served from.

use super::*;

#[test]
fn a_layout_is_served_by_the_smallest_class_that_holds_its_size_and_alignment() {
    assert_eq!(class_of(1, 1), Some(0));
    assert_eq!(class_of(32, 8), Some(0));
    assert_eq!(class_of(33, 8), Some(1));
    assert_eq!(class_of(4096, 8), Some(CLASSES - 1));
    assert_eq!(class_of(4097, 8), None, "past the largest class, the lists");
    assert_eq!(class_of(40, 64), Some(1), "the alignment chooses when it is larger");
    assert_eq!(class_of(8, 4096), Some(CLASSES - 1));
    assert_eq!(class_of(8, 8192), None);
    assert_eq!(class_of(0, 8), None);
    for size in 1..=5000 {
        for align in [8, 16, 32, 64, 128, 4096] {
            match class_of(size, align) {
                Some(c) => {
                    let s = CLASS_SIZES[c];
                    assert!(size <= s && align <= s, "{size} {align}");
                    assert!(c == 0 || size > CLASS_SIZES[c - 1] || align > CLASS_SIZES[c - 1]);
                }
                None => assert!(size > 4096 || align > 4096),
            }
            assert_eq!(class_of(size, align), class_of(size, align), "alloc and free agree");
        }
    }
}

#[test]
fn the_kernels_common_small_blocks_are_all_in_a_class() {
    // An IPC name String, a pending-reply entry, a BTreeMap node of the page
    // record, a run-queue snapshot: each with the heap's wrapper.
    for (what, size) in [("name", 24), ("entry", 64), ("node", 544), ("pids", 32), ("vec", 2048)] {
        assert!(class_of(size + WRAP, 8).is_some(), "{what}");
    }
    // An IPC chunk (60 KiB) goes to the lists, where only large blocks are.
    assert_eq!(class_of(60 * 1024 + 8 + WRAP, 8), None);
}
