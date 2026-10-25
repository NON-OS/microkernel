// NONOS Operating System (AGPL-3.0-or-later)
//! The clock a terminal runs on, over seeded 2.0 functions, sound and then
//! told one lie: clock() finds what the rule in clock_sweep/model.rs finds
//! and what the lie must leave.

use crate::clock_sweep::lie::{tell, LIES};
use crate::clock_sweep::model;
use crate::clock_sweep::rng::{rounds, Rng};
use crate::uac::clock;

#[test]
fn after_any_lie_a_terminal_runs_on_the_clock_its_function_says() {
    let (mut told, mut found) = (vec![0u32; LIES.len()], 0);
    for seed in 1..rounds(30_000) {
        let (lie, c, t, want) = tell(&mut Rng::new(seed));
        let at = format!("seed {seed}: {lie:?}, terminal {t}");
        let got = clock(&c, t).map(|k| (k.interface, k.id, k.programmable, k.selector));
        assert_eq!(got, model::clock(&c, t), "{at}: the rule");
        assert_eq!(got, want, "{at}");
        (told[LIES.iter().position(|l| *l == lie).unwrap()], found) =
            (told[LIES.iter().position(|l| *l == lie).unwrap()] + 1, found + got.is_some() as u32);
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 2_000, "{lie:?} was told only {n} times");
    }
    assert!(cfg!(miri) || found >= 5_000, "only {found} clocks found");
}
