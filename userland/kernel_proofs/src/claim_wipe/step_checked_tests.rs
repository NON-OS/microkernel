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

//! What a check makes of a written reset, over every phase, answer, marker
//! reading and fallback, the clock either side of the deadline: Clean only
//! back as itself with the marker gone; back with it set, the fallback if
//! any, else Unreset; not back, Lost only at or past the deadline or again.

use super::end_rule::{checked, End, Next};
use super::method_rule::Method::{self, AfFlr, Flr, PmCycle};
use super::phase_rule::{Phase, FLR_WAIT, MS, PM_WAIT, RETRY_WINDOW};
use super::step_cases::phases;

const FALLS: [Option<Method>; 4] = [None, Some(Flr), Some(AfFlr), Some(PmCycle)];

/// From the windows themselves, not the rule's helper, so a wrong one shows.
fn deadline(p: Phase) -> Option<u64> {
    match p {
        Phase::Flr { at } => Some(at.saturating_add(FLR_WAIT + RETRY_WINDOW)),
        Phase::D0 { at } => Some(at.saturating_add(PM_WAIT + RETRY_WINDOW)),
        _ => None,
    }
}

/// Every phase begun early, midway and late, read around its start and end.
fn cases() -> Vec<(Phase, u64, Option<Method>)> {
    let mut all = Vec::new();
    for at in [0, 3 * MS + 1, u64::MAX - FLR_WAIT] {
        for p in phases(at) {
            let d = deadline(p).unwrap_or(at.saturating_add(FLR_WAIT + RETRY_WINDOW));
            let nows = [at.checked_sub(1), Some(at), Some(d - 1), Some(d), d.checked_add(1)];
            all.extend(nows.into_iter().flatten().flat_map(|now| FALLS.map(|f| (p, now, f))));
        }
    }
    all
}

#[test]
fn clean_only_when_back_and_unmarked_and_lost_only_past_the_deadline() {
    // Clean, retried, unreset; lost again, lost late; short of it, marker clear.
    let mut seen = [0u32; 7];
    for (p, now, fall) in cases() {
        for (ready, confirmed) in [(false, false), (false, true), (true, false), (true, true)] {
            let want = match (ready, confirmed, fall) {
                (true, true, _) => Next::Ended(End::Clean),
                (true, false, Some(m)) => Next::Retry(m),
                (true, false, None) => Next::Ended(End::Unreset),
                (false, ..) if p == Phase::Recheck => Next::Ended(End::Lost),
                (false, ..) if deadline(p).is_some_and(|d| now >= d) => Next::Ended(End::Lost),
                (false, ..) => Next::Stay,
            };
            assert_eq!(checked(p, now, ready, confirmed, fall), want, "{p:?} {now} {fall:?}");
            seen[0] += (want == Next::Ended(End::Clean)) as u32;
            seen[1] += matches!(want, Next::Retry(_)) as u32;
            seen[2] += (want == Next::Ended(End::Unreset)) as u32;
            seen[3 + (p != Phase::Recheck) as usize] += (want == Next::Ended(End::Lost)) as u32;
            seen[5] += (want == Next::Stay && deadline(p).is_some_and(|d| d - 1 == now)) as u32;
            seen[6] += (want == Next::Stay && confirmed) as u32;
        }
    }
    assert_eq!(seen, [416, 312, 104, 104, 80, 48, 324]);
}
