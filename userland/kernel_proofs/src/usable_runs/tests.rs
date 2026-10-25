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

use super::runs::{Runs, ROOM};

#[test]
fn touching_and_overlapping_regions_merge_and_ends_are_exclusive() {
    let mut r = Runs::EMPTY;
    for (s, e) in [(0x1000, 0x9F000), (0x10_0000, 0x7B80_0000), (0x7B80_0000, 0x7B90_0000)] {
        r.add(s, e);
    }
    r.add(0x7000_0000, 0x7100_0000);
    assert_eq!(&r.runs[..r.len], &[(0x1000, 0x9F000), (0x10_0000, 0x7B90_0000)]);
    assert!(!r.touches(0x9F000, 0x10_0000), "the gap between runs");
    assert!(r.touches(0x9E000, 0x9F001) && r.touches(0x7B8F_F000, 0x7C00_0000));
    assert!(!r.touches(0x7B90_0000, 0x7F80_0000), "the first byte past a run");
    r.add(0x2000, 0x1000);
    assert_eq!(r.len, 2, "an empty or inverted region adds nothing");
}

#[test]
fn past_the_room_kept_every_region_touches() {
    let mut r = Runs::EMPTY;
    for i in 0..ROOM as u64 {
        r.add(i * 0x2000, i * 0x2000 + 0x1000);
    }
    assert!(!r.full && !r.touches(0x1000, 0x2000));
    r.add(ROOM as u64 * 0x2000, ROOM as u64 * 0x2000 + 0x1000);
    assert!(r.full && r.touches(u64::MAX - 1, u64::MAX), "unknown reads as touching");
}
