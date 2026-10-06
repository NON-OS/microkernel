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

use super::room::{has_room, MAX_GUESTS};

#[test]
fn a_supervisor_stops_at_its_ceiling() {
    let mut held = 0;
    while has_room(held) {
        held += 1;
        assert!(held <= MAX_GUESTS, "the count passed the ceiling");
    }
    assert_eq!(held, MAX_GUESTS);
}

#[test]
fn the_ceiling_leaves_two_linux_families_room() {
    // capsule_linux holds each family to 512 tasks.
    const { assert!(MAX_GUESTS >= 2 * 512) };
    assert!(has_room(MAX_GUESTS - 1));
    assert!(!has_room(MAX_GUESTS));
    assert!(!has_room(usize::MAX));
}
