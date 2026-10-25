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

//! Whether a claim may begin over a function's wipe record, every phase by both services by the
//! kept-frames bound by the clock behind the loss and either side of the lost hold: no record, go,
//! but refused past the bound; a claim standing or a wipe under way, ask again; kept, its own
//! service only, under the bound; lost, refused for its hold, then asked once more. Which service
//! asks matters for a kept record and nothing else.

use super::admit_rule::{admits, Admit};
use super::phase_rule::{Phase, LOST_HOLD, MS};
use super::step_cases::phases;

const KINDS: [Admit; 4] = [Admit::Go, Admit::Rearm, Admit::Resetting, Admit::Refused];

/// The table in the plan's order, with no catch-all arm.
fn want(record: Option<Phase>, same: bool, now: u64, over_cap: bool) -> Admit {
    let go = [Admit::Go, Admit::Refused][over_cap as usize];
    match record {
        None => go,
        Some(Phase::Kept) => [Admit::Refused, go][same as usize],
        Some(Phase::Lost { at }) if now < at || now - at < LOST_HOLD => Admit::Refused,
        Some(Phase::Lost { .. }) => Admit::Rearm,
        Some(Phase::Held | Phase::Owed { .. } | Phase::Flr { .. }) => Admit::Resetting,
        Some(Phase::D3 { .. } | Phase::D0 { .. } | Phase::Recheck) => Admit::Resetting,
    }
}

#[test]
fn the_full_table_over_every_phase_service_bound_and_hold_edge() {
    // Go, rearmed, resetting, refused; refused behind the loss; turned by the service.
    let mut seen = [0u32; 6];
    for at in [LOST_HOLD, 40 * MS + 7, u64::MAX - LOST_HOLD - 1] {
        let nows = [at - 1, at, at + LOST_HOLD - 1, at + LOST_HOLD, at + LOST_HOLD + 1];
        let records = [None].into_iter().chain(phases(at).map(Some));
        for (record, now) in records.flat_map(|r| nows.map(|n| (r, n))) {
            for over_cap in [false, true] {
                let by = [false, true].map(|same| admits(record, same, now, over_cap));
                for same in [false, true] {
                    let got = by[same as usize];
                    assert_eq!(got, want(record, same, now, over_cap), "{record:?} {now} {same}");
                    seen[KINDS.iter().position(|&k| k == got).unwrap_or(3)] += 1;
                    seen[4] += (record == Some(Phase::Lost { at }) && now < at && !over_cap) as u32;
                }
                let turned = by[0] != by[1];
                assert_eq!(turned, record == Some(Phase::Kept) && !over_cap, "{record:?}");
                seen[5] += turned as u32;
            }
        }
    }
    assert_eq!(seen, [45, 24, 360, 111, 6, 15]);
}

#[test]
fn a_lost_function_is_refused_for_its_hold_and_asked_once_more_after() {
    let lost = Some(Phase::Lost { at: 5 * MS });
    assert_eq!(admits(lost, true, 5 * MS + LOST_HOLD - 1, false), Admit::Refused);
    assert_eq!(admits(lost, false, 5 * MS + LOST_HOLD, false), Admit::Rearm);
    assert_eq!(admits(lost, true, 5 * MS - 1, false), Admit::Refused, "a clock behind it");
    assert_eq!(admits(lost, true, u64::MAX, true), Admit::Rearm, "asked again past the bound");
    assert_eq!(admits(None, true, 0, true), Admit::Refused, "a new tenancy past the bound");
}
