// NONOS Operating System (AGPL-3.0-or-later)
//! The stream a function plays and the one it captures, over seeded
//! configurations, sound and then with one alternate told a lie: find
//! takes what the rule in alt_sweep/model.rs takes, and the first of the
//! others that plays or captures as stated.

use crate::alt_sweep::lie::{tell, LIES};
use crate::alt_sweep::model;
use crate::capture_sweep::rng::{rounds, Rng};
use crate::uac::find;

#[test]
fn after_any_lie_the_first_alternate_that_plays_or_captures_is_taken() {
    let mut told = vec![0u32; LIES.len()];
    let mut found = 0;
    for seed in 1..rounds(40_000) {
        let (lie, c, want) = tell(&mut Rng::new(seed));
        for (play, want) in [(true, want[0]), (false, want[1])] {
            let at = format!("seed {seed}: {lie:?}, play {play}");
            let got = find(&c, play);
            assert_eq!(got, model::find(&c, play), "{at}: the rule");
            assert_eq!(got, want, "{at}");
            found += got.is_some() as u32;
        }
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_000, "{lie:?} was told only {n} times");
    }
    assert!(cfg!(miri) || found >= 4_000, "only {found} streams found");
}
