// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Which pass the follow-up wait counts its floor from (server/runner/
//! pace.rs paced_from). A title-bar drag's step is two requests: the
//! pointer moved, which on the cursor plane draws nothing, then the
//! window's scene submit. The pass that served the pointer used to start the
//! floor, so the submit's drain waited out the rest of it, and the timer
//! sweep after it, before composing: 5 to 10 ms of each drag frame on a
//! development guest. Counted from the last pass that drew, the submit is
//! composed at once, frames that draw are still at most one a floor, and
//! pointer moves alone still reach the plane at most once a floor.

use super::pace_sim::{drag_steps, every, mean, run_mixed, Mixed, NO_WAIT, TICK_US};
use crate::runner::pace::{follow_up, paced_from, Last, FRAME_FLOOR_MS};

fn new_rule(now: i64, served: i64, drew: i64, damaged: bool) -> u64 {
    follow_up(now, paced_from(Last { served, drew }, damaged)).unwrap_or(NO_WAIT)
}

/// Before: every pass that served input or drew started the floor.
fn old_rule(now: i64, served: i64, _: i64, _: bool) -> u64 {
    follow_up(now, served).unwrap_or(NO_WAIT)
}

/// Times are read in whole ms, so a gap can be up to 1 ms short.
const FLOOR_US: u64 = (FRAME_FLOOR_MS as u64 - 1) * 1000;

fn spaced(at: &[u64]) -> bool {
    at.windows(2).all(|p| p[1] - p[0] >= FLOOR_US)
}

fn worst(r: &Mixed) -> u64 {
    r.draw_latency_us.iter().copied().max().unwrap_or(0)
}

#[test]
fn a_drain_with_damage_counts_from_the_last_pass_that_drew() {
    let last = Last { served: 900, drew: 100 };
    assert_eq!(paced_from(last, true), 100);
    assert_eq!(paced_from(last, false), 900);
    // A pointer pass 2 ms ago, the last frame 50 ms ago: a submit only looks.
    assert_eq!(follow_up(1002, paced_from(Last { served: 1000, drew: 952 }, true)), None);
    assert_eq!(follow_up(1002, paced_from(Last { served: 1000, drew: 952 }, false)), Some(6));
}

#[test]
fn a_drag_step_is_composed_at_once_not_after_its_pointers_floor() {
    for phase in (0..TICK_US).step_by(700) {
        for behind in [300u64, 1_500, 4_000] {
            let at = drag_steps(100_000, 50_000 + phase, behind, 20);
            let (old, new) = (run_mixed(old_rule, &at), run_mixed(new_rule, &at));
            assert_eq!(new.draw_latency_us.len(), 20);
            assert!(worst(&new) <= 1_100, "phase {phase}, behind {behind}: {} us", worst(&new));
            assert!(worst(&old) > 4_000, "phase {phase}, behind {behind}: the old wait");
            assert!(mean(&new.draw_latency_us) < mean(&old.draw_latency_us));
        }
    }
}

#[test]
fn a_fast_drag_still_draws_at_most_once_a_floor_and_never_later() {
    for hz in [60u64, 125, 250, 500, 1000] {
        let at = drag_steps(1_000_000 / hz, 3_300, 400, (hz * 2) as usize);
        let (old, new) = (run_mixed(old_rule, &at), run_mixed(new_rule, &at));
        assert!(spaced(&new.frames_at_us), "{hz} Hz: frames closer than a floor");
        assert!(mean(&new.draw_latency_us) <= mean(&old.draw_latency_us), "{hz} Hz");
        assert!(worst(&new) <= worst(&old), "{hz} Hz: {} > {}", worst(&new), worst(&old));
    }
}

#[test]
fn pointer_moves_alone_reach_the_plane_at_most_once_a_floor() {
    for hz in [125u64, 250, 500, 1000] {
        let at: Vec<(u64, bool)> = every(1_000_000 / hz, 3_300, (hz * 2) as usize)
            .into_iter()
            .map(|t| (t, false))
            .collect();
        let (old, new) = (run_mixed(old_rule, &at), run_mixed(new_rule, &at));
        assert!(new.frames_at_us.is_empty());
        assert!(spaced(&new.plane_at_us), "{hz} Hz: plane passes closer than a floor");
        assert_eq!(new.plane_at_us, old.plane_at_us, "{hz} Hz: pointer alone paced as before");
    }
}

/// The numbers this benchmark reports:
/// cargo test --release drag_pace_numbers -- --ignored --nocapture
#[test]
#[ignore]
fn drag_pace_numbers() {
    for (name, period) in
        [("drag 10 Hz", 100_000u64), ("drag 60 Hz", 16_667), ("drag 125 Hz", 8_000)]
    {
        let at = drag_steps(period, 53_300, 1_500, 400);
        for (rule, f) in [("before", old_rule as super::pace_sim::PacedRule), ("after", new_rule)] {
            let r = run_mixed(f, &at);
            println!(
                "{name} {rule}: submit to frame done mean {} us, worst {} us, {} frames",
                mean(&r.draw_latency_us),
                worst(&r),
                r.frames_at_us.len()
            );
        }
    }
}
