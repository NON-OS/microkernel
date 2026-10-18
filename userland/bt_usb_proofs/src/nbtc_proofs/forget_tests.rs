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

//! FORGET: the bond dropped, its levels with it, and its link ended.

use super::bond_tests::body;
use super::fixture::{ask, board, bonded, KB, MOUSE};
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::Settings;
use nonos_bt_hogp::host::Battery;

#[test]
fn forget_drops_the_bond_its_levels_and_its_link() {
    let mut b = board();
    bonded(&mut b, KB, true);
    b.linked(KB, false);
    b.battery(Battery { instance: 0, percent: 70 });
    assert_eq!(ask(&mut b, Settings, OP_FORGET, &body(KB), 0), (E_OK, vec![]));
    assert!(b.drop_link, "its link is to end");
    assert!(b.levels.is_empty());
    assert!(b.bonds.book().bonds().is_empty());
    assert_eq!(ask(&mut b, Settings, OP_FORGET, &body(KB), 0).0, E_NOENT);
}

#[test]
fn forget_leaves_another_link_alone() {
    let mut b = board();
    bonded(&mut b, KB, true);
    bonded(&mut b, MOUSE, false);
    b.linked(MOUSE, false);
    assert_eq!(ask(&mut b, Settings, OP_FORGET, &body(KB), 0).0, E_OK);
    assert!(!b.drop_link);
    assert_eq!(ask(&mut b, Settings, OP_LINK, &[], 0).1[2], 0, "the mouse moved up the list");
}
