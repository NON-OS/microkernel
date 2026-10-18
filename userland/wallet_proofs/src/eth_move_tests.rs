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
//! Moving NOX at a fresh address into the Shield (v1.4.56): the review read
//! ETH found at a fresh address moves into the Shield too (v1.4.60), paying
//! its own gas: a row moves its NOX first, then its ETH, and says which.

use crate::shield::fresh_move_act::{label_of, moves, Act};
use crate::shield::landed_held::Held;

#[test]
fn a_row_moves_its_nox_first_then_its_eth() {
    assert_eq!(moves(Held { nox: 5, eth: 9 }), Some((false, 5)), "NOX first");
    assert_eq!(moves(Held { nox: 0, eth: 9 }), Some((true, 9)), "then the ETH");
    assert_eq!(moves(Held { nox: 0, eth: 0 }), None, "nothing to move");
    assert_eq!(moves(Held { nox: u128::MAX, eth: u128::MAX }), Some((false, u128::MAX)));
}

#[test]
fn the_button_names_eth_only_when_eth_moves() {
    assert_eq!(label_of(Act::Move, true), "Move the ETH to the Shield");
    assert_eq!(label_of(Act::Move, false), "Move to the Shield");
    let deposits = Act::Confirm { approval: false };
    assert_eq!(label_of(deposits, true), "Confirm the deposits");
}
