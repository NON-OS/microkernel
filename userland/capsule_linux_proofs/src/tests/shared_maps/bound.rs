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

//! The table holds at most 64 maps: one more is ENOMEM, and so is a munmap
//! whose split would make one more, while a munmap that splits nothing may
//! always give maps back.

use super::support::{map, Entry, P};
use crate::linux::abi::errno::ENOMEM;
use crate::linux::file::shared_maps::rules::{admits, unmap, MAX_MAPS};

/// `n` readers of one file, three pages each, a page apart, in process 1.
fn full(n: u64) -> Vec<Entry> {
    (0..n).map(|i| map(1, 16 + 4 * i, 3, "/f", 0)).collect()
}

#[test]
fn the_table_stops_at_64_maps() {
    assert_eq!(MAX_MAPS, 64);
    let new = map(2, 16, 1, "/f", 0);
    assert_eq!(admits(&full(63), &new, false), Ok(()));
    assert_eq!(admits(&full(64), &new, false), Err(ENOMEM));
    assert_eq!(admits(&full(64), &map(1, 16, 3, "/f", 0), true), Ok(()), "one replaced");
    assert_eq!(admits(&full(64), &map(1, 16, 2, "/f", 0), true), Err(ENOMEM), "one cut");
}

#[test]
fn a_split_past_64_is_refused_and_nothing_else_is() {
    let hole = (16 * P + P, P);
    assert_eq!(unmap(&full(64), 1, hole.0, hole.1), Err(ENOMEM), "the 65th by a split");
    assert_eq!(unmap(&full(63), 1, hole.0, hole.1).map(|t| t.len()), Ok(64));
    assert_eq!(unmap(&full(64), 1, 16 * P, 3 * P).map(|t| t.len()), Ok(63), "one given back");
    assert_eq!(unmap(&full(64), 1, 16 * P, P).map(|t| t.len()), Ok(64), "one cut short");
    assert_eq!(unmap(&full(64), 2, hole.0, hole.1), Ok(full(64)), "another process's");
}
