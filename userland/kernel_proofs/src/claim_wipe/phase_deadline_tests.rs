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

//! When a written reset is given up on (L78), and the windows' lengths. The
//! deadline lies one full retry window past where its check opens, and a step
//! just before, at or after it is a check, so the step that finds it not back
//! by then is the one that can end it Lost. Only a written FLR and the step
//! back to D0 have one, saturating near the clock's end; an owed reset and a
//! recheck need none, being due at once at any clock.

use super::due_rule::{deadline, due, Due};
use super::phase_rule::{Phase, FLR_WAIT, LOST_HOLD, MS, PM_WAIT, RETRY_WINDOW, TP_BOUND};
use super::phase_tests::draw;

#[test]
fn the_windows_are_the_lengths_the_plan_names() {
    // FLR's quiet time, the retry window and the D3hot step (recalled, unverified);
    // the Transactions Pending bound and the lost hold are ours.
    assert_eq!((MS, FLR_WAIT, RETRY_WINDOW, PM_WAIT), (1_000_000, 100 * MS, 1000 * MS, 10 * MS));
    assert_eq!((TP_BOUND, LOST_HOLD), (100 * MS, 2000 * MS));
}

#[test]
fn the_deadline_lies_a_retry_window_past_where_its_check_opens() {
    let (mut x, mut unaligned) = (0x6A09_E667_F3BC_C909u64, 0u32);
    for _ in 0..20_000 {
        let at = draw(&mut x) >> 2;
        for (p, wait) in [(Phase::Flr { at }, FLR_WAIT), (Phase::D0 { at }, PM_WAIT)] {
            let d = deadline(p).expect("a written reset has a deadline");
            assert_eq!(d, at + wait + RETRY_WINDOW, "{p:?}");
            assert_eq!(due(p, d - RETRY_WINDOW - 1), Due::Wait, "{p:?} before its check opens");
            for now in [d - RETRY_WINDOW, d - 1, d, d + 1] {
                assert_eq!(due(p, now), Due::Check, "{p:?} at {now}, deadline {d}");
            }
        }
        let rest = [Phase::Held, Phase::Owed { since: at }, Phase::D3 { at }, Phase::Recheck];
        for p in rest.into_iter().chain([Phase::Kept, Phase::Lost { at }]) {
            assert_eq!(deadline(p), None, "{p:?}");
        }
        unaligned += u32::from(!at.is_multiple_of(MS));
    }
    assert!(unaligned > 19_000, "{unaligned} starts off a millisecond");
    let edge = |wait: u64| [u64::MAX - wait - RETRY_WINDOW - 1, u64::MAX - wait - RETRY_WINDOW + 1];
    let (flr, d0) = (edge(FLR_WAIT), edge(PM_WAIT));
    assert_eq!(flr.map(|at| deadline(Phase::Flr { at })), [Some(u64::MAX - 1), Some(u64::MAX)]);
    assert_eq!(d0.map(|at| deadline(Phase::D0 { at })), [Some(u64::MAX - 1), Some(u64::MAX)]);
    assert_eq!(deadline(Phase::Flr { at: u64::MAX }), Some(u64::MAX), "saturates, never wraps");
}

#[test]
fn an_owed_reset_and_a_recheck_are_due_at_once_at_any_clock() {
    let (mut x, mut behind) = (0xBB67_AE85_84CA_A73Bu64, 0u32);
    for _ in 0..20_000 {
        let (since, now) = (draw(&mut x), draw(&mut x));
        assert_eq!(due(Phase::Owed { since }, now), Due::Start, "owed {since}, read {now}");
        assert_eq!(due(Phase::Recheck, now), Due::Check, "read {now}");
        behind += u32::from(now < since);
    }
    assert!(behind > 9_000, "{behind} clocks behind the release");
}
