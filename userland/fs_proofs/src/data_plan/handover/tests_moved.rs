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


//! The first boot under a new bootloader (layout.rs `moved`): of its two
//! writes, the first holds way 3 wrapped again and the record still good,
//! the second the new wrap alone. Whichever one the disk last took, some
//! boot opens the volume: the old bootloader's or the record's before the
//! first, the new one's after it.

use super::super::key_header::{parse_key_header, Keyed};
use super::super::key_wrap::{unwrap as unwrap3, wrap as wrap3};
use super::layout::{moved, read};
use super::seal::unwrap;
use super::tests::{prepared, HAND, KEY, OWN};

/// The new bootloader's own key.
const NEW: [u8; 32] = [11; 32];

fn way3_opens(s: &[u8; 512], kek: &[u8; 32]) -> Option<[u8; 32]> {
    match parse_key_header(s) {
        Ok(Some(Keyed::Approved(w))) => unwrap3(kek, &w),
        _ => None,
    }
}

#[test]
fn the_first_write_keeps_the_record_and_the_second_clears_it() {
    let (s, r) = prepared();
    let new = wrap3(&NEW, &KEY, [3; 12]).unwrap();
    let (kept, cleared) = moved(&s, &new);
    assert_eq!(way3_opens(&kept, &NEW), Some(KEY));
    assert_eq!(read(&kept), Some(r));
    assert_eq!(unwrap(&HAND, &r, &kept), Some(KEY), "the record still opens");
    assert_eq!(way3_opens(&cleared, &NEW), Some(KEY));
    assert_eq!(read(&cleared), None);
    assert!(cleared[116..].iter().all(|&b| b == 0));
}

#[test]
fn every_state_the_disk_can_be_left_in_opens() {
    let (s, r) = prepared();
    let (kept, cleared) = moved(&s, &wrap3(&NEW, &KEY, [3; 12]).unwrap());
    for (name, state) in [("before", s), ("kept", kept), ("cleared", cleared)] {
        let by_own = way3_opens(&state, &NEW).or_else(|| way3_opens(&state, &OWN));
        let by_record = read(&state).and_then(|r| unwrap(&HAND, &r, &state));
        assert_eq!(by_own.or(by_record), Some(KEY), "{name}");
    }
    assert_eq!(way3_opens(&s, &NEW), None, "before, only the record opens it here");
    assert_eq!(unwrap(&HAND, &r, &s), Some(KEY));
}
