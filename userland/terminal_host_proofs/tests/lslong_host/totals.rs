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

//! The `total` line: sum, human units and saturation.

use crate::ls_long::Row;
use crate::ls_total::total_line;

fn file(size: u64) -> Row {
    Row { name: b"f".to_vec(), size, is_dir: false, mtime: 0, writable: true }
}

fn total(sizes: &[u64], human: bool) -> String {
    let rows: Vec<Row> = sizes.iter().map(|&s| file(s)).collect();
    String::from_utf8(total_line(&rows, human)).expect("the total is ascii")
}

#[test]
fn the_total_is_the_summed_byte_size() {
    assert_eq!(total(&[], false), "total 0");
    assert_eq!(total(&[1, 2, 3], false), "total 6");
    assert_eq!(total(&[4096, 900], false), "total 4996");
}

#[test]
fn the_total_follows_the_human_flag() {
    assert_eq!(total(&[1024, 1024], true), "total 2.0K");
    assert_eq!(total(&[1_048_576], true), "total 1.0M");
    assert_eq!(total(&[512], true), "total 512B");
}

/// A directory big enough to overflow the sum would otherwise wrap to a small
/// total, which reads as truth.
#[test]
fn the_total_saturates_instead_of_wrapping() {
    assert_eq!(total(&[u64::MAX, u64::MAX], false), total(&[u64::MAX], false));
}
