// NONOS Operating System (AGPL-3.0-or-later)
//! F-126: no event that arrived while its GPE was off is lost. Its status
//! stays latched, and the SCI after the re-arm serves it; a level source
//! still asserting is served then, once.

use super::clock::Clock;
use super::fixture::scene;
use super::never_tests::TICK;
use crate::events::{CUTS, FIRST_MS};
use crate::tests::sci_rig::bits;

#[test]
fn an_event_raised_while_off_is_served_at_the_re_arm() {
    for seed in [0, 1, 10, 11] {
        // Raised at the window's first tick, its middle, its last, or at
        // every tick of it (a level source that never let go).
        for when in [Some(TICK), Some(FIRST_MS / 2), Some(FIRST_MS - TICK), None] {
            let mut c = Clock::new(&scene(seed));
            let mut now = 0;
            for _ in 0..CUTS {
                c.tick(now, true, 1);
                now += TICK;
            }
            let off = now - TICK;
            assert!(!c.rig.events.armed.contains(&c.g), "seed {seed}: off");
            while now < off + FIRST_MS {
                let raise = when.is_none_or(|w| now == off + w);
                let t = c.tick(now, raise, 1_000_000);
                assert!(t.report.ran.is_empty(), "seed {seed} {when:?}: nothing while off");
                assert!(bits(&c.rig.regs, c.g).0 || now < off + when.unwrap_or(TICK));
                now += TICK;
            }
            let t = c.tick(now, false, 1_000_000);
            assert_eq!(t.rearmed, [c.g], "seed {seed} {when:?}: armed again");
            assert_eq!(t.report.ran.len(), 1, "seed {seed} {when:?}: served at the re-arm");
            assert!(t.report.ran[0].1.is_ok() && t.report.disabled.is_empty());
            assert_eq!(bits(&c.rig.regs, c.g), (false, true), "seed {seed}: cleared, armed");
            let t = c.tick(now + TICK, false, 1_000_000);
            assert!(t.report.ran.is_empty(), "seed {seed} {when:?}: served once");
        }
    }
}
