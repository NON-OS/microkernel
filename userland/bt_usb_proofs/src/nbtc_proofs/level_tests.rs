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

//! Levels of a link that kept no bond, and the bound on how many are kept.

use super::fixture::{ask, board, bonded, KB};
use crate::nbtc::ops::OP_BATTERY;
use crate::nbtc::part::LEVELS_MAX;
use crate::nbtc::who::Caller::Shell;
use nonos_bt_hogp::host::Battery;

#[test]
fn a_link_with_no_bond_has_its_levels_until_it_ends() {
    let mut b = board();
    b.battery(Battery { instance: 0, percent: 55 });
    assert_eq!(ask(&mut b, Shell, OP_BATTERY, &[], 0).1, [1, 0xFF, 0, 55]);
    b.ended();
    assert_eq!(ask(&mut b, Shell, OP_BATTERY, &[], 0).1, [0]);
}

#[test]
fn levels_past_the_bound_are_not_kept() {
    let mut b = board();
    for i in 0..=LEVELS_MAX as u8 {
        b.battery(Battery { instance: i, percent: 1 });
    }
    assert_eq!(b.levels.len(), LEVELS_MAX);
    assert_eq!(ask(&mut b, Shell, OP_BATTERY, &[], 0).1[0] as usize, LEVELS_MAX);
}

#[test]
fn a_level_whose_bond_went_another_way_is_left_out() {
    let mut b = board();
    bonded(&mut b, KB, true);
    b.linked(KB, false);
    b.battery(Battery { instance: 0, percent: 30 });
    // The link drops a bond the peer no longer holds (`again.rs`) itself.
    b.bonds.forget(KB.0, &KB.1).unwrap();
    assert_eq!(ask(&mut b, Shell, OP_BATTERY, &[], 0).1, [0]);
}
