// NONOS Operating System (AGPL-3.0-or-later)
//! F-126: a method that recovers is served again within `LAST_MS` of
//! recovering, and its finished run clears both the count and the window.

use super::clock::Clock;
use super::fixture::scene;
use super::never_tests::TICK;
use crate::events::{CUTS, FIRST_MS, LAST_MS};
use crate::tests::sci_rig::bits;

const BIG: u64 = 1_000_000;

#[test]
fn a_method_that_recovers_is_served_within_the_longest_window() {
    let heals = [0, 500, 1_020, 5_000, 63_000, 130_000, 200_000, 260_000];
    for (seed, heal) in [0, 1, 10, 11].into_iter().flat_map(|s| heals.map(|h| (s, h))) {
        let mut c = Clock::new(&scene(seed));
        let mut now = 0;
        let served = loop {
            let t = c.tick(now, true, if now < heal { 1 } else { BIG });
            if t.report.ran.iter().any(|(_, r)| r.is_ok()) {
                break now;
            }
            assert!(now < heal + LAST_MS, "seed {seed} heal {heal}: not served by {now}");
            now += TICK;
        };
        assert!(served >= heal && served - heal <= LAST_MS, "seed {seed} heal {heal}");
        assert!(bits(&c.rig.regs, c.g).1 && c.rig.events.armed.contains(&c.g));
        // The finished run cleared the count: CUTS - 1 cuts in a row leave
        // it armed; the next sends it off for the first window, not a longer.
        for k in 1..=u64::from(CUTS) {
            now += TICK;
            let t = c.tick(now, true, 1);
            assert_eq!(t.report.ran.len(), 1, "seed {seed} heal {heal}: armed for cut {k}");
            let off = if k < u64::from(CUTS) { vec![] } else { vec![(c.g, Some(FIRST_MS))] };
            assert_eq!(t.report.disabled, off, "seed {seed} heal {heal}: cut {k}");
        }
    }
}
