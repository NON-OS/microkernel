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

//! BONDS and FORGET's payload: identities, never keys.

use super::fixture::{ask, board, bonded, KB, LTK, MOUSE};
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::Settings;

pub fn body(who: (u8, [u8; 6])) -> Vec<u8> {
    [&[who.0][..], &who.1].concat()
}

#[test]
fn the_bond_list_gives_identities_and_how_they_were_made() {
    let mut b = board();
    bonded(&mut b, KB, true);
    bonded(&mut b, MOUSE, false);
    b.linked(MOUSE, false);
    let (status, list) = ask(&mut b, Settings, OP_BONDS, &[], 0);
    assert_eq!(status, E_OK);
    let kb = [&body(KB)[..], &[1, 0]].concat();
    let mouse = [&body(MOUSE)[..], &[0, 1]].concat();
    assert_eq!(list, [&[2u8][..], &kb, &mouse].concat());
    assert!(!list.windows(4).any(|w| w == &LTK[..4]), "no key leaves the capsule");
}

#[test]
fn forget_takes_exactly_an_identity() {
    let mut b = board();
    bonded(&mut b, KB, true);
    let mut bad_type = body(KB);
    bad_type[0] = 2;
    for wrong in [vec![], body(KB)[..6].to_vec(), [&body(KB)[..], &[0]].concat(), bad_type] {
        assert_eq!(ask(&mut b, Settings, OP_FORGET, &wrong, 0).0, E_INVAL, "{wrong:?}");
    }
    assert_eq!(b.bonds.book().bonds().len(), 1);
}
