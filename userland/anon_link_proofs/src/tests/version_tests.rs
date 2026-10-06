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

//! Link version negotiation against what the relay offered.

use crate::link::versions::negotiate;

#[test]
fn the_highest_common_version_wins() {
    let offered = [0u8, 3, 0, 4, 0, 5];
    assert_eq!(negotiate(&offered), Some(5));
}

#[test]
fn order_does_not_decide() {
    assert_eq!(negotiate(&[0, 5, 0, 4]), Some(5));
    assert_eq!(negotiate(&[0, 4, 0, 5]), Some(5));
}

#[test]
fn no_overlap_has_no_answer() {
    assert_eq!(negotiate(&[0, 1, 0, 2, 0, 3]), None);
    assert_eq!(negotiate(&[]), None);
}

#[test]
fn a_truncated_list_ignores_its_tail() {
    assert_eq!(negotiate(&[0, 5, 0]), Some(5));
}
