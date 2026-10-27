// NONOS Operating System (AGPL-3.0-or-later)
//! A long churn of small blocks against the first-fit lists.

use super::*;

/// The stall's shape: small blocks of mixed sizes and lifetimes freed between
/// one another for a long time. The first-fit lists see one slab request per
/// class per SLAB_BYTES of the most that class ever held, and nothing else,
/// so no churn of small blocks can cut them into holes.
#[test]
fn a_long_churn_of_small_blocks_reaches_the_lists_only_for_slabs() {
    let mut classes = SizeClasses::new();
    let mut arena = Arena::new(1024);
    let mut live: Vec<(usize, *mut u8)> = Vec::new();
    let mut held = [0usize; CLASSES];
    let mut peak = [0usize; CLASSES];
    let mut rng = 0x9e37_79b9_7f4a_7c15u64;
    for step in 0..400_000u64 {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let size = 1 + (rng % 2000) as usize + WRAP;
        let class = class_of(size, 8).expect("small");
        // Mostly alloc early, mostly free late: the heap fills and drains.
        let free_now = !live.is_empty() && (rng >> 32) % 100 < if step < 200_000 { 45 } else { 55 };
        if free_now {
            let i = (rng >> 40) as usize % live.len();
            let (c, block) = live.swap_remove(i);
            unsafe { classes.push(c, block) };
            held[c] -= 1;
        } else {
            live.push((class, take(&mut classes, &mut arena, class)));
            held[class] += 1;
            peak[class] = peak[class].max(held[class]);
        }
    }
    let bound: usize = (0..CLASSES).map(|c| (peak[c] * CLASS_SIZES[c]).div_ceil(SLAB_BYTES)).sum();
    assert!(arena.taken <= bound, "slabs {} past the bound {bound}", arena.taken);
    let in_use: usize = live.iter().map(|(c, _)| CLASS_SIZES[*c]).sum();
    assert_eq!(in_use + classes.free_bytes(), arena.taken * SLAB_BYTES, "every byte accounted");
}
