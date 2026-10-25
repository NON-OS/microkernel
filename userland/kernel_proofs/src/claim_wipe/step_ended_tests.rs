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

//! What a wipe's end leaves behind. Kept frames go after a reset unless the
//! function reaches memory past Bus Master; after no reset only with Bus
//! Master read back off as well; never while it may yet come back. The
//! record goes only after a reset of a function that is not a display, so a
//! claimant of another service is let in after that end and no other.

use super::admit_rule::{admits, Admit};
use super::end_rule::{ended, frees, End};
use super::phase_rule::{Phase, LOST_HOLD};

const ENDS: [End; 3] = [End::Clean, End::Unreset, End::Lost];

#[test]
fn kept_frames_go_only_where_nothing_can_reach_them() {
    // (bme_off, binds) in the order (off, no), (on, no), (off, yes), (on, yes).
    let table = [
        (End::Clean, [true, true, false, false]),
        (End::Unreset, [true, false, false, false]),
        (End::Lost, [false; 4]),
    ];
    for (end, row) in table {
        for (i, (bme_off, binds)) in
            [(true, false), (false, false), (true, true), (false, true)].into_iter().enumerate()
        {
            assert_eq!(frees(end, bme_off, binds), row[i], "{end:?} off {bme_off} binds {binds}");
        }
    }
}

#[test]
fn the_record_goes_only_after_a_reset_of_a_function_that_is_not_a_display() {
    for now in [0, 123, u64::MAX] {
        assert_eq!(ended(End::Clean, false, now), None);
        assert_eq!(
            ended(End::Clean, true, now),
            Some(Phase::Kept),
            "may outlive a reset (recalled, unverified)"
        );
        for display in [false, true] {
            assert_eq!(ended(End::Unreset, display, now), Some(Phase::Kept));
            assert_eq!(ended(End::Lost, display, now), Some(Phase::Lost { at: now }));
        }
    }
}

#[test]
fn another_service_is_let_in_after_a_clean_reset_of_a_non_display_and_no_other_end() {
    let now = 7_000;
    for (end, display) in ENDS.into_iter().flat_map(|e| [(e, false), (e, true)]) {
        let left = ended(end, display, now);
        let stranger = admits(left, false, now, false);
        let own = admits(left, true, now, false);
        let clean = end == End::Clean && !display;
        assert_eq!(stranger == Admit::Go, clean, "{end:?} display {display}");
        assert_eq!(own == Admit::Go, end != End::Lost, "{end:?} display {display}");
        let later = admits(left, false, now + LOST_HOLD, false);
        assert_eq!(later == Admit::Rearm, end == End::Lost, "a lost one is asked once more");
    }
}
