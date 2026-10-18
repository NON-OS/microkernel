// NONOS Operating System (AGPL-3.0-or-later)
//! F-126: a method the budget cuts every time costs one budget per back-off
//! window once `CUTS` runs in a row were cut, and the window doubles from
//! `FIRST_MS` up to `LAST_MS`; so the loop never spins on it.

use super::clock::Clock;
use super::fixture::scene;
use crate::events::{CUTS, FIRST_MS, LAST_MS};

/// The loop's tick, in ms; our choice, from no source.
pub const TICK: u64 = 10;
/// How long each scene runs: past several capped windows.
const SPAN: u64 = 400_000;

/// The windows in order, from the ruling: doubling from the first, capped.
pub fn windows() -> impl Iterator<Item = u64> {
    let mut w = FIRST_MS;
    std::iter::from_fn(move || {
        let now = w;
        w = (w * 2).min(LAST_MS);
        Some(now)
    })
}

#[test]
fn a_method_that_never_ends_runs_once_per_window() {
    for seed in [0, 1, 10, 11] {
        let mut c = Clock::new(&scene(seed));
        let (mut runs, mut offs, mut back) = (Vec::new(), Vec::new(), Vec::new());
        for now in (0..SPAN).step_by(TICK as usize) {
            let t = c.tick(now, true, 1);
            back.extend(t.rearmed.iter().map(|_| now));
            for (n, r) in &t.report.ran {
                assert_eq!(*n, c.g);
                assert!(r.as_ref().is_err_and(|f| f.is_budget()), "seed {seed}: cut at {now}");
                runs.push(now);
            }
            offs.extend(t.report.disabled.iter().map(|(n, w)| (now, *n, *w)));
        }
        let first: Vec<u64> = (0..u64::from(CUTS)).map(|k| k * TICK).collect();
        assert_eq!(runs[..first.len()], first[..], "seed {seed}: armed until {CUTS} cuts");
        // Each run after those is the one at a re-arm, and goes off again.
        let mut at = first[first.len() - 1];
        let mut want = (Vec::new(), Vec::new());
        for w in windows() {
            want.0.push((at, c.g, Some(w)));
            at += w;
            if at >= SPAN {
                break;
            }
            want.1.push(at);
        }
        assert_eq!(offs, want.0, "seed {seed}: one window each, doubling to the cap");
        assert_eq!(runs[first.len()..], want.1[..], "seed {seed}: one run per window");
        assert_eq!(back, want.1, "seed {seed}: armed again as each window ends");
        assert!(runs.windows(2).skip(first.len() - 1).all(|p| p[1] - p[0] >= FIRST_MS));
    }
}
