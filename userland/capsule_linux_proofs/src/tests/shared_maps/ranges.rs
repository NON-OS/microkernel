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

//! The two ranges of a map, and which bytes of each a write, a cut, a
//! munmap and a read reach.

use super::support::{map, P};
use crate::linux::file::shared_maps::entry::{clip, overlap};
use crate::linux::file::shared_maps::spans::{echo, going, zeroed};

#[test]
fn ranges_meet_only_where_they_share_a_byte() {
    assert_eq!(overlap((0, 10), (10, 20)), None, "touching is not meeting");
    assert_eq!(overlap((0, 10), (20, 30)), None);
    assert_eq!(overlap((0, 30), (10, 20)), Some((10, 20)));
    assert_eq!(overlap((5, 15), (10, 20)), Some((10, 15)));
    assert_eq!(overlap((7, 7), (0, 20)), None, "an empty range meets nothing");
}

#[test]
fn a_range_is_cut_at_the_file_end() {
    assert_eq!(clip((0, P), 5000), Some((0, P)));
    assert_eq!(clip((P, 3 * P), 5000), Some((P, 5000)));
    assert_eq!(clip((2 * P, 3 * P), 5000), None, "wholly past the end is nothing");
    assert_eq!(clip((0, P), 0), None, "an empty file has nothing to clip to");
}

#[test]
fn a_munmap_reaches_the_file_bytes_its_pages_show() {
    let e = map(1, 16, 4, "/f", 3);
    assert_eq!(e.file(), (3 * P, 7 * P));
    assert_eq!(e.addr(5 * P + 7), 18 * P + 7, "two pages in, seven bytes in");
    assert_eq!(going(&e, 17 * P, P), Some((4 * P, 5 * P)));
    assert_eq!(going(&e, 10 * P, 7 * P), Some((3 * P, 4 * P)), "a hole over its start");
    assert_eq!(going(&e, 19 * P, 10 * P), Some((6 * P, 7 * P)), "a hole over its end");
    assert_eq!(going(&e, 20 * P, P), None, "the page after it");
    assert_eq!(going(&e, 0, u64::MAX), Some(e.file()), "a span that wraps is cut, not wrapped");
}

#[test]
fn a_write_lands_where_the_map_shows_its_bytes() {
    let e = map(1, 16, 2, "/f", 1);
    assert_eq!(echo(&e, P + 10, 5), Some((16 * P + 10, 0, 5)));
    assert_eq!(echo(&e, P - 3, 8), Some((16 * P, 3, 8)), "begun before the map");
    assert_eq!(echo(&e, 3 * P - 2, 8), Some((18 * P - 2, 0, 2)), "running past it");
    assert_eq!(echo(&e, 0, P), None, "the page before it");
    assert_eq!(echo(&e, 3 * P, 1), None, "the byte after it");
    assert_eq!(echo(&e, u64::MAX - 1, 9), None, "an end that would wrap");
}

#[test]
fn a_cut_or_a_growth_zeroes_what_lies_between_the_ends() {
    let e = map(1, 16, 2, "/f", 0);
    assert_eq!(zeroed(&e, 2 * P, 100), Some((16 * P + 100, 2 * P - 100)), "shrunk");
    assert_eq!(zeroed(&e, 100, P), Some((16 * P + 100, P - 100)), "grown");
    assert_eq!(zeroed(&e, 5 * P, 3 * P), None, "both ends past the map");
    assert_eq!(zeroed(&e, 70, 70), None, "no change of length");
    assert_eq!(zeroed(&e, 0, u64::MAX), Some((16 * P, 2 * P)), "the whole map, no more");
}
