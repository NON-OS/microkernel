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

//! The numbers the shield service keeps earlier spends under, matched to
//! the spends this window proved: the newest unknown numbers are this
//! window's, in order, and any older ones are followed as spends it did not
//! see proved.

use crate::kept_names::split;

#[test]
fn this_windows_spends_take_the_newest_numbers_in_order() {
    let (older, newer) = split(&[], 2, &["0", "1"]);
    assert!(older.is_empty());
    assert_eq!(newer, ["0", "1"]);
}

#[test]
fn spends_from_before_a_restart_are_the_older_ones() {
    let (older, newer) = split(&[], 1, &["3", "4", "5"]);
    assert_eq!(older, ["3", "4"]);
    assert_eq!(newer, ["5"]);
}

#[test]
fn a_known_number_is_never_given_again() {
    let (older, newer) = split(&["4"], 1, &["4", "5"]);
    assert!(older.is_empty());
    assert_eq!(newer, ["5"]);
    let (older, newer) = split(&["4", "5"], 0, &["4", "5"]);
    assert!(older.is_empty() && newer.is_empty());
}

#[test]
fn a_spend_the_service_does_not_name_yet_stays_unnamed() {
    let (older, newer) = split(&[], 2, &["7"]);
    assert!(older.is_empty());
    assert_eq!(newer, ["7"], "only one number for two spends: the newer is named next time");
}
