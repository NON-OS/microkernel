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

//! The `.` and `..` rows.

use crate::ls_dots::{dot_names, dot_rows};
use crate::ls_long;

fn names(dots: &[Vec<u8>]) -> Vec<String> {
    dots.iter().map(|n| String::from_utf8(n.clone()).unwrap()).collect()
}

#[test]
fn dots_appear_only_under_dash_a() {
    assert!(dot_names(b"/home/", false).is_empty());
    assert!(dot_names(b"/", false).is_empty());
    assert_eq!(names(&dot_names(b"/home/", true)), vec!["./", "../"]);
}

/// The root has no parent to name, so `..` there would point at itself.
#[test]
fn the_root_has_no_parent_row() {
    assert_eq!(names(&dot_names(b"/", true)), vec!["./"]);
}

#[test]
fn dot_rows_claim_nothing_the_vfs_did_not_report() {
    let rows = dot_rows(&dot_names(b"/home/", true));
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert!(row.is_dir);
        assert_eq!(row.size, 0);
        assert_eq!(row.mtime, 0);
        assert!(!row.writable);
    }
}

#[test]
fn a_dot_row_renders_as_a_directory_with_no_date() {
    let rows = dot_rows(&dot_names(b"/home/", true));
    let line = String::from_utf8(ls_long::long_row(&rows[0], false)).unwrap();
    assert_eq!(line, "dr--         0            - ./");
}
