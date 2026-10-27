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

//! One writer per range of a file: a map that is or may become a writer
//! shares no byte of its file with another map, in any process.

use super::support::{map, may, writer};
use crate::linux::abi::errno::ENOSYS;
use crate::linux::file::shared_maps::rules::admits;

#[test]
fn a_writer_meets_no_other_map_of_its_bytes_in_any_process() {
    let all = vec![writer(map(1, 16, 4, "/f", 0))];
    assert_eq!(admits(&all, &writer(map(2, 16, 2, "/f", 3)), false), Err(ENOSYS));
    assert_eq!(admits(&all, &map(2, 16, 2, "/f", 3), false), Err(ENOSYS), "nor a reader");
    assert_eq!(admits(&all, &map(1, 40, 1, "/f", 0), false), Err(ENOSYS), "its own either");
    assert_eq!(admits(&all, &writer(map(2, 16, 2, "/f", 4)), false), Ok(()), "past its bytes");
    assert_eq!(admits(&all, &writer(map(2, 16, 4, "/g", 0)), false), Ok(()), "another file");
}

#[test]
fn a_map_that_may_become_a_writer_counts_as_one() {
    let all = vec![may(map(1, 16, 4, "/f", 0))];
    assert_eq!(admits(&all, &map(2, 16, 1, "/f", 1), false), Err(ENOSYS));
    let all = vec![map(1, 16, 4, "/f", 0)];
    assert_eq!(admits(&all, &may(map(2, 16, 1, "/f", 1)), false), Err(ENOSYS));
}

#[test]
fn readers_of_a_file_stand_side_by_side() {
    let all = vec![map(1, 16, 4, "/f", 0), map(2, 16, 4, "/f", 0)];
    assert_eq!(admits(&all, &map(3, 16, 4, "/f", 0), false), Ok(()));
    assert_eq!(admits(&all, &map(1, 40, 4, "/f", 0), false), Ok(()), "twice in one process");
}

#[test]
fn map_fixed_over_its_own_map_replaces_it_and_over_another_does_not() {
    let all = vec![writer(map(1, 16, 4, "/f", 0)), map(2, 16, 4, "/g", 0)];
    let new = writer(map(1, 16, 4, "/f", 0));
    assert_eq!(admits(&all, &new, true), Ok(()), "MAP_FIXED unmaps the old one first");
    assert_eq!(admits(&all, &new, false), Err(ENOSYS));
    assert_eq!(admits(&all, &writer(map(2, 16, 4, "/f", 0)), true), Err(ENOSYS));
}
