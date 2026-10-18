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

//! BONDS and BATTERY at their fullest, read with the shared crate's client
//! half.

use super::client_call::call;
use super::fixture::{board, bonded};
use crate::nbtc::ops::*;
use nonos_bt_hogp::host::Battery;
use nonos_bt_proto as p;

#[test]
fn a_full_bond_list_and_its_levels_read_back() {
    let mut b = board();
    for i in 0..nonos_bt_bond::MAX_BONDS as u8 {
        bonded(&mut b, (0, [i, 1, 2, 3, 4, 5]), i % 2 == 0);
    }
    b.linked((0, [3, 1, 2, 3, 4, 5]), false);
    b.battery(Battery { instance: 1, percent: 100 });
    let data = call(&mut b, OP_BONDS, &[], 0).1;
    let bonds = p::Bonds::read(&data).unwrap();
    assert_eq!(bonds.len(), p::BONDS_MAX);
    let third = bonds.get(3).unwrap();
    assert_eq!((third.addr[0], third.authenticated, third.linked), (3, false, true));
    assert!(bonds.get(2).unwrap().authenticated && !bonds.get(2).unwrap().linked);
    assert_eq!(bonds.get(p::BONDS_MAX), None);
    let data = call(&mut b, OP_BATTERY, &[], 0).1;
    let levels = p::Levels::read(&data).unwrap();
    assert_eq!(levels.get(0), Some(p::Level { place: 3, instance: 1, percent: 100 }));
    assert_eq!(levels.len(), 1);
}
