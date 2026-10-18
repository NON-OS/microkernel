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

//! A faulting device's id is noted once, without a lock, and taken once
//! where the broker can block it (K12).

use super::noted::Noted;

#[test]
fn an_id_is_noted_once_and_taken_once() {
    let noted: Noted<4> = Noted::new();
    assert!(noted.note(0x0300) && noted.note(0x0300) && noted.note(0x0010));
    let mut taken: Vec<u16> = noted.take().collect();
    taken.sort();
    assert_eq!(taken, [0x0010, 0x0300]);
    assert_eq!(noted.take().count(), 0, "taken ids are gone");
}

#[test]
fn id_zero_and_the_top_id_are_ids_like_any_other() {
    let noted: Noted<2> = Noted::new();
    assert!(noted.note(0) && noted.note(u16::MAX));
    let mut taken: Vec<u16> = noted.take().collect();
    taken.sort();
    assert_eq!(taken, [0, u16::MAX]);
}

#[test]
fn a_full_table_refuses_a_new_id_and_frees_on_take() {
    let noted: Noted<2> = Noted::new();
    assert!(noted.note(1) && noted.note(2));
    assert!(!noted.note(3), "full: the next fault notes it again");
    assert!(noted.note(2), "one already held is still noted");
    assert_eq!(noted.take().count(), 2);
    assert!(noted.note(3));
}
