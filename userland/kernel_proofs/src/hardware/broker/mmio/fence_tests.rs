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

//! A departed device's windows stay fenced while the space that may map
//! them lives: a newcomer inside one is caught, the fence follows the
//! tables to a thread that inherits them, and lifts when they are freed.

use super::fence::{after_exit, fence, fenced_by, overlaps, Fenced};

#[test]
fn ranges_overlap_only_where_they_share_a_byte() {
    assert!(overlaps(0x1000, 0x1000, 0x1fff, 1));
    assert!(!overlaps(0x1000, 0x1000, 0x2000, 0x1000), "touching ends");
    assert!(!overlaps(0x1000, 0, 0x1000, 0x1000), "empty window");
    assert!(overlaps(u64::MAX - 1, 10, u64::MAX - 1, 1), "no wrap");
    assert!(overlaps(0x1000, 0x100, 0x800, 0x1000), "containing");
}

#[test]
fn a_fence_follows_the_tables_then_lifts() {
    fence(Fenced { pid: 41, start: 0xfe00_0000, len: 0x4000 });
    fence(Fenced { pid: 41, start: 0xfe00_0000, len: 0x4000 });
    assert_eq!(fenced_by(0xfe00_2000, 0x1000), Some(41));
    assert_eq!(fenced_by(0xfe00_4000, 0x1000), None);
    after_exit(41, Some(42));
    assert_eq!(fenced_by(0xfe00_0000, 1), Some(42), "a thread kept the tables");
    after_exit(41, None);
    assert_eq!(fenced_by(0xfe00_0000, 1), Some(42), "another pid's exit lifts nothing");
    after_exit(42, None);
    assert_eq!(fenced_by(0xfe00_0000, 1), None);
}
