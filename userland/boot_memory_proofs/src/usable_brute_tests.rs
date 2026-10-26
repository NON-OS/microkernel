// NONOS Operating System (AGPL-3.0-or-later)
//! Random ranges against a page bitmap: the runs never name a page no range
//! added, and while they fit they name every one; they stay sorted and
//! never meet or touch.

use crate::usable::{Usable, MAX_RUNS};

const PAGES: usize = 512;

fn next(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *seed >> 33
}

#[test]
fn runs_match_the_bitmap() {
    let mut seed = 7u64;
    for round in 0..4000 {
        let (mut u, mut bits) = (Usable::EMPTY, [false; PAGES]);
        for _ in 0..(round % 48) {
            let a = next(&mut seed) % PAGES as u64;
            let b = a + next(&mut seed) % 24;
            u.add(a * 0x1000, b.min(PAGES as u64) * 0x1000);
            (a..b.min(PAGES as u64)).for_each(|p| bits[p as usize] = true);
        }
        let runs = u.runs();
        assert!(runs.windows(2).all(|w| w[0].1 < w[1].0), "round {round}: {runs:x?}");
        let mut named = [false; PAGES];
        for &(a, b) in runs {
            (a / 0x1000..b / 0x1000).for_each(|p| named[p as usize] = true);
        }
        for p in 0..PAGES {
            assert!(!named[p] || bits[p], "round {round}: page {p} named but never added");
        }
        if runs.len() < MAX_RUNS {
            assert_eq!(named, bits, "round {round}: a page left out with room to spare");
        }
    }
}
