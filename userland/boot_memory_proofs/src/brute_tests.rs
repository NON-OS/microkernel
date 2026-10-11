// NONOS Operating System (AGPL-3.0-or-later)
//! `place` against a search of every page, over many small layouts.

use crate::dma_place::place;

const PAGE: u64 = 0x1000;

/// The lowest page-aligned run a search of every page finds.
fn search(available: &[(u64, u64)], avoid: &[(u64, u64)], floor: u64, want: u64) -> Option<u64> {
    (0..96u64).map(|p| p * PAGE).find(|&at| {
        at >= floor
            && available.iter().any(|&(s, e)| at >= s && at + want <= e)
            && !avoid.iter().any(|&(a, b)| a < at + want && at < b)
    })
}

/// A small deterministic stream of layouts.
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

fn range(seed: &mut u64) -> (u64, u64) {
    let a = next(seed) % 0x3_8000;
    (a, a + next(seed) % 0x1_8000)
}

#[test]
fn place_is_the_lowest_clear_run_a_full_search_finds() {
    let mut seed = 0x9e37_79b9_7f4a_7c15;
    for _ in 0..20_000 {
        let available = [range(&mut seed), range(&mut seed)];
        let avoid = [range(&mut seed), range(&mut seed), range(&mut seed)];
        let floor = next(&mut seed) % 0x1_0000;
        let want = (1 + next(&mut seed) % 6) * PAGE;
        let got = place(&available, &avoid, floor, want);
        assert_eq!(got, search(&available, &avoid, floor, want), "{available:x?} {avoid:x?}");
    }
}
