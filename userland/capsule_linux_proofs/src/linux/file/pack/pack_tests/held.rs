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

//! A swap that finds the old pack held: refused while the new pack names
//! other files, in place already when it begins with the same header and
//! index, and never mistaken for in place with no head or no hold.

use super::super::swap_head::in_place;
use super::*;

/// `pack` swapped in over a held "old!": what the swap and the check say,
/// and the store after.
fn over_held(pack: &[u8], head: usize) -> (Result<(), Held>, Model) {
    let mut m = Model { held: Some(vim()), ..with_old() };
    m.files.insert(vim(), b"old!".to_vec());
    let why = swap(&mut m, &vim(), pack, &minted(pack)).expect_err("the pack is held");
    (in_place(&mut m, &vim(), pack, head, why), m)
}

#[test]
fn a_held_pack_that_begins_the_same_is_in_place_and_serves_on() {
    let (put, m) = over_held(b"old!", 3);
    assert_eq!(put, Ok(()));
    assert_eq!(m.files.get(&vim()), Some(&b"old!".to_vec()), "the held pack is untouched");
    assert!(!m.files.contains_key(&new_path(&vim())), "no .new is left");
    assert_eq!(over_held(b"oldX", 3).0, Ok(()), "the head names every file by digest");
}

#[test]
fn a_held_pack_that_names_other_files_stays_refused() {
    assert_eq!(over_held(b"new!", 3).0, Err(Held::InUse));
    assert_eq!(over_held(b"olX!", 3).0, Err(Held::InUse));
    let (put, m) = over_held(b"new!", 4);
    assert_eq!(put, Err(Held::InUse));
    assert_eq!(m.files.get(&vim()), Some(&b"old!".to_vec()));
}

#[test]
fn no_head_or_no_hold_is_never_in_place() {
    assert_eq!(over_held(b"old!", 0).0, Err(Held::InUse), "no head names nothing");
    assert_eq!(over_held(b"old!", 5).0, Err(Held::InUse), "a head past the pack's end");
    let unwritten = in_place(&mut with_old(), &vim(), b"old", 3, Held::Unwritten);
    assert_eq!(unwritten, Err(Held::Unwritten), "only a hold is checked");
}
