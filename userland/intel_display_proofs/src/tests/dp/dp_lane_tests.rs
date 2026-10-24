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

//! Drive levels and the lane status bytes.

use crate::dp::lane::{Drive, LaneStatus};

#[test]
fn drive_levels_stay_inside_the_standards_table() {
    assert_eq!(Drive { swing: 3, pre: 2 }.legal(), Drive { swing: 3, pre: 0 });
    assert_eq!(Drive { swing: 1, pre: 3 }.legal(), Drive { swing: 1, pre: 2 });
    assert_eq!(Drive { swing: 9, pre: 9 }.legal(), Drive { swing: 3, pre: 0 });
}

#[test]
fn lane_set_flags_the_maximum_reached() {
    assert_eq!(Drive { swing: 0, pre: 0 }.lane_set(), 0);
    assert_eq!(Drive { swing: 2, pre: 1 }.lane_set(), 2 | (1 << 3) | (1 << 5));
    assert_eq!(Drive { swing: 3, pre: 0 }.lane_set(), 3 | (1 << 2) | (1 << 5));
}

#[test]
fn status_bytes_answer_per_lane_and_alignment() {
    let s = LaneStatus([0x77, 0x17, 0x01, 0, 0, 0]);
    assert!(s.cr_done(4) && !s.eq_done(4), "lane 3 not locked");
    assert!(s.eq_done(3));
    let unaligned = LaneStatus([0x77, 0x77, 0x00, 0, 0, 0]);
    assert!(!unaligned.eq_done(4));
    assert!(
        LaneStatus([0x01, 0, 0, 0, 0, 0]).cr_done(1)
            && !LaneStatus([0x10, 0, 0, 0, 0, 0]).cr_done(1)
    );
}

#[test]
fn the_drive_asked_for_is_the_highest_of_the_lanes_used() {
    // Lane 0 swing 1, lane 1 pre 2, lane 2 swing 2 pre 1, lane 3 swing 3.
    let s = LaneStatus([0, 0, 0, 0, 0x01 | (0x08 << 4), 0x06 | (0x03 << 4)]);
    assert_eq!(s.requested(2), Drive { swing: 1, pre: 2 });
    assert_eq!(s.requested(4), Drive { swing: 3, pre: 0 }, "clamped to the table");
}
