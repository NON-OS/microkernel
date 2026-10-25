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

//! The keyring's refusals, as the client hands them to the bond store.

use crate::ring::keyring::Call;
use crate::ring::keyring::Keyring;
use crate::ring_sim::{capsule, ring, Line, NO_ANSWER};
use nonos_bt_bond::{Keys, KeysError, KIND_BOND};

#[test]
fn the_keyrings_refusals_are_named() {
    let ring = ring(true);
    let mut keys = capsule(&ring, 7);
    for _ in 0..16 {
        keys.store(KIND_BOND, &[1; 48]).unwrap();
    }
    assert_eq!(keys.store(KIND_BOND, &[1; 48]), Err(KeysError::Full));
    // A full owner's list still fits the room the client gives it.
    assert_eq!(keys.list(KIND_BOND).map(|ids| ids.len()), Ok(16));
    assert_eq!(keys.delete(999), Err(KeysError::NotFound));
    assert_eq!(keys.retrieve(999), Err(KeysError::NotFound));
    // A capsule that names a pid not its own is refused (`caller.rs`).
    let mut lying = Keyring::new(Line(ring, 8), 7);
    assert_eq!(lying.list(KIND_BOND), Err(KeysError::Denied));
}

#[test]
fn an_answer_too_long_for_its_room_is_no_answer() {
    let ring = ring(true);
    let mut keys = capsule(&ring, 7);
    let id = keys.store(KIND_BOND, &[1; 65]).unwrap();
    assert_eq!(keys.retrieve(id), Err(KeysError::Unavailable));
    let mut line = Line(ring, 7);
    let asked = line.call(2, &[&7u32.to_le_bytes()[..], &id.to_le_bytes()].concat(), 64);
    assert_eq!(asked, Err(NO_ANSWER));
}
