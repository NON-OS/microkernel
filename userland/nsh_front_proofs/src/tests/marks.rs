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

//! A role table or revocation list older than one already served under is
//! refused, and the newest one admitted becomes the mark.

use crate::marks::Marks;

#[test]
fn the_first_records_are_admitted() {
    let mut m = Marks::new();
    assert_eq!(m.serial(), 0);
    assert!(m.admit(3, 1_000));
    assert_eq!(m.serial(), 3);
}

#[test]
fn the_same_or_newer_records_are_admitted_and_older_are_not() {
    let mut m = Marks::new();
    assert!(m.admit(3, 1_000));
    assert!(m.admit(3, 1_000), "the same records again");
    assert!(m.admit(3, 2_000), "a fresher list");
    assert!(!m.admit(3, 1_999), "an older list put back");
    assert!(!m.admit(2, 5_000), "an older table, whatever its list");
    assert!(m.admit(4, 2_000), "a newer table");
    assert!(!m.admit(3, 9_000));
    assert_eq!(m.serial(), 4);
}

#[test]
fn a_refusal_leaves_the_marks_as_they_were() {
    let mut m = Marks::new();
    assert!(m.admit(5, 5_000));
    let before = m;
    assert!(!m.admit(4, 6_000));
    assert!(!m.admit(6, 4_000));
    assert_eq!(m, before);
}
