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

//! Which phases a step moves on (L78). Held, Kept and Lost are never due at
//! any clock, so no step touches a function a claim holds, nor one kept or
//! lost until a claim asks again. `pending`, on which the table's count and
//! the refusal of a second record rest, holds for exactly the phases some
//! clock makes due; a written reset always leaves a pending phase and an end
//! never does, so the count the step reads first falls to 0 only when no
//! record is left to step.

use super::due_rule::{due, pending, Due};
use super::end_rule::{ended, End};
use super::method_rule::Method;
use super::phase_rule::{Phase, FLR_WAIT, PM_WAIT, RETRY_WINDOW};
use super::phase_tests::draw;
use super::start_rule::written;

fn phases(at: u64) -> [Phase; 8] {
    let (owed, flr, d3, d0) =
        (Phase::Owed { since: at }, Phase::Flr { at }, Phase::D3 { at }, Phase::D0 { at });
    [Phase::Held, owed, flr, d3, d0, Phase::Recheck, Phase::Kept, Phase::Lost { at }]
}

#[test]
fn only_the_stepped_phases_are_pending_and_held_kept_lost_never_come_due() {
    let (mut x, mut seen) = (0xD1B5_4A32_D192_ED03u64, [0u32; 2]);
    for _ in 0..5_000 {
        let at = draw(&mut x) >> 2;
        let gaps = [0, PM_WAIT, FLR_WAIT, FLR_WAIT + RETRY_WINDOW, 1 << 62, draw(&mut x) >> 2];
        let mut clocks = gaps.map(|g| at + g).to_vec();
        clocks.extend([0, at.saturating_sub(1), u64::MAX, draw(&mut x)]);
        for p in phases(at) {
            let stepped = clocks.iter().any(|&now| due(p, now) != Due::Wait);
            assert_eq!(pending(p), stepped, "{p:?}");
            let never = matches!(p, Phase::Held | Phase::Kept | Phase::Lost { .. });
            assert_eq!(pending(p), !never, "{p:?}");
            seen[usize::from(stepped)] += 1;
        }
    }
    assert_eq!(seen, [3 * 5_000, 5 * 5_000], "{seen:?}");
}

#[test]
fn a_written_reset_leaves_a_pending_phase_and_an_end_leaves_none() {
    let (mut x, mut ends) = (0x8CB9_2BA7_2F3D_8DD7u64, 0u32);
    for _ in 0..1_000 {
        let now = draw(&mut x) >> 2;
        for m in [Method::Flr, Method::AfFlr, Method::PmCycle] {
            assert!(pending(written(m, now)), "{m:?} written, then never stepped");
        }
        for (end, display) in
            [End::Clean, End::Unreset, End::Lost].map(|e| [(e, false), (e, true)]).concat()
        {
            let left = ended(end, display, now);
            assert!(!left.is_some_and(pending), "{end:?} display {display} left {left:?}");
            ends += u32::from(left.is_some());
        }
    }
    assert_eq!(ends, 5 * 1_000, "every end but a clean non-display one leaves a record");
}
