// NONOS Operating System (AGPL-3.0-or-later)
//! A slab cut into blocks, and a freed block handed out again.

use super::*;

#[test]
fn a_slab_is_cut_into_disjoint_aligned_blocks_handed_out_once_each() {
    for class in 0..CLASSES {
        let mut classes = SizeClasses::new();
        let mut arena = Arena::new(2);
        let size = CLASS_SIZES[class];
        let per_slab = SLAB_BYTES / size;
        let mut got: Vec<usize> =
            (0..per_slab).map(|_| take(&mut classes, &mut arena, class) as usize).collect();
        assert_eq!(arena.taken, 1, "one slab serves {per_slab} blocks of {size}");
        assert_eq!(classes.free_bytes(), 0);
        got.sort_unstable();
        got.dedup();
        assert_eq!(got.len(), per_slab, "no block twice");
        for w in got.windows(2) {
            assert!(w[1] - w[0] >= size, "blocks do not overlap");
        }
        assert!(got.iter().all(|b| b % size.min(SLAB_ALIGN) == 0), "aligned to the class");
        take(&mut classes, &mut arena, class);
        assert_eq!(arena.taken, 2, "the next one cuts a second slab");
        assert_eq!(classes.free_bytes(), SLAB_BYTES - size);
    }
}

#[test]
fn a_freed_block_is_the_next_one_handed_out() {
    let mut classes = SizeClasses::new();
    let mut arena = Arena::new(1);
    let a = take(&mut classes, &mut arena, 3);
    let b = take(&mut classes, &mut arena, 3);
    let before = classes.free_bytes();
    unsafe { classes.push(3, a) };
    assert_eq!(classes.free_bytes(), before + CLASS_SIZES[3]);
    assert_eq!(take(&mut classes, &mut arena, 3), a);
    assert_ne!(take(&mut classes, &mut arena, 3), b);
    assert_eq!(arena.taken, 1);
}
