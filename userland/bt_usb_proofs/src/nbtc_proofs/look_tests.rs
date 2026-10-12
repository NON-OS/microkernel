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

//! HEALTH, LINK and BATTERY: what the link last told the board, read as
//! the shell reads it, with places in the bond list and no addresses.

use super::fixture::{ask, board, bonded, KB, MOUSE};
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::Shell;
use nonos_bt_hogp::host::Battery;

#[test]
fn health_gives_the_version_and_the_controller() {
    let mut b = board();
    assert_eq!(ask(&mut b, Shell, OP_HEALTH, &[], 0), (E_OK, vec![1, 0, 1]));
    b.controller = false;
    assert_eq!(ask(&mut b, Shell, OP_HEALTH, &[], 0), (E_OK, vec![1, 0, 0]));
    assert_eq!(ask(&mut b, Shell, OP_LINK, &[], 0).1[0], NO_CONTROLLER);
}

#[test]
fn link_follows_the_link_from_scan_to_ready_and_back() {
    let mut b = board();
    bonded(&mut b, MOUSE, false);
    bonded(&mut b, KB, true);
    b.state = SCANNING;
    assert_eq!(ask(&mut b, Shell, OP_LINK, &[], 0).1, [SCANNING, 0b1000, 0xFF]);
    b.state = SECURING;
    b.linked(KB, false);
    b.ready(true, false, true);
    assert_eq!(ask(&mut b, Shell, OP_LINK, &[], 0).1, [READY, 0b1101, 1], "keyboard, keys post");
    b.ended();
    assert_eq!(ask(&mut b, Shell, OP_LINK, &[], 0).1, [SCANNING, 0b1000, 0xFF]);
    b.scan = false;
    b.ended();
    assert_eq!(ask(&mut b, Shell, OP_LINK, &[], 0).1, [IDLE, 0, 0xFF]);
}

#[test]
fn levels_are_kept_by_bond_and_by_battery_service() {
    let mut b = board();
    bonded(&mut b, KB, true);
    b.linked(KB, false);
    b.battery(Battery { instance: 0, percent: 90 });
    b.battery(Battery { instance: 1, percent: 40 });
    b.battery(Battery { instance: 0, percent: 89 });
    b.ended();
    assert_eq!(
        ask(&mut b, Shell, OP_BATTERY, &[], 0).1,
        [2, 0, 0, 89, 0, 1, 40],
        "kept past the link"
    );
}
