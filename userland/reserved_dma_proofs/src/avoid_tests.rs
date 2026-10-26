// NONOS Operating System (AGPL-3.0-or-later)
//! Placement never lands on a reserved range: over generated reserved
//! spans, bump pointers and lengths, the run it gives is clear of every
//! span, obeys `place`'s own rules, and exists whenever any clear run does.

use crate::avoid::place_avoiding;
use crate::iova_space::{after_give_back, place, IOVA_LIMIT};

#[test]
fn a_grant_never_meets_a_reserved_range() {
    let mut seed = 41u64;
    let mut next = |bound: u64| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) % bound
    };
    let mut placed = 0;
    for _ in 0..100_000 {
        let page = 0x1000;
        let span = if next(8) == 0 { IOVA_LIMIT } else { 0x40_0000 };
        let reserved: Vec<(u64, u64)> = (0..next(5))
            .map(|_| {
                let s = next(span / page) * page;
                (s, s + (1 + next(64)) * page)
            })
            .collect();
        let (bump, length) = (next(span / page) * page, (1 + next(256)) * page);
        let clear = |s: u64, e: u64| reserved.iter().all(|&(rs, re)| e <= rs || re <= s);
        match place_avoiding(bump, length, &reserved) {
            Some((start, end)) => {
                assert!(clear(start, end), "{start:#x}..{end:#x} meets {reserved:x?}");
                assert_eq!(place(start, length), Some((start, end)), "not a run `place` gives");
                assert!(start >= bump && end - start == length);
                placed += 1;
            }
            None => {
                // No clear run at the bump pointer or past any span's end.
                let tries = std::iter::once(bump).chain(reserved.iter().map(|&(_, e)| e.max(bump)));
                for at in tries {
                    if let Some((s, e)) = place(at, length) {
                        assert!(!clear(s, e), "refused, yet {s:#x}..{e:#x} is clear");
                    }
                }
            }
        }
    }
    assert!(placed > 50_000, "hardly any placed: {placed}");
}

/// A run given back lowers the bump pointer only when it is the top one,
/// so a run below it is never handed out a second time from the top.
#[test]
fn only_the_top_run_given_back_lowers_the_pointer() {
    assert_eq!(after_give_back(0x9000, 0x8000, 0x1000), 0x8000, "the top run");
    assert_eq!(after_give_back(0x9000, 0x4000, 0x1000), 0x9000, "a run below");
    assert_eq!(after_give_back(0x9000, u64::MAX, 2), 0x9000, "a run past the end");
}
