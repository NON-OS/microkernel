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

//! Who may ask what: Settings everything, the shell the link's state and
//! battery levels only, anyone else nothing; by the sender's pid against
//! the pid that holds each name now.

use super::fixture::{ask, board};
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::{Other, Settings, Shell};
use crate::nbtc::who::{caller, may};

const OPS: [u16; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0x77];

fn owners(name: &[u8]) -> Option<u32> {
    match name {
        b"app.settings.1" => Some(40),
        b"desktop_shell" => Some(41),
        b"app.terminal" => Some(42),
        _ => None,
    }
}

#[test]
fn the_caller_is_whoever_holds_the_name_now() {
    assert_eq!(caller(40, owners), Settings, "a later Settings window");
    assert_eq!(caller(41, owners), Shell);
    assert_eq!(caller(42, owners), Other);
    assert_eq!(caller(0, |_| Some(0)), Other, "the kernel holds no name here");
    assert_eq!(caller(43, owners), Other);
}

#[test]
fn the_shell_reads_the_link_and_battery_and_nothing_else() {
    let allowed: Vec<u16> = OPS.into_iter().filter(|&op| may(Shell, op)).collect();
    assert_eq!(allowed, [OP_HEALTH, OP_LINK, OP_BATTERY]);
    let mut b = board();
    for op in [OP_BONDS, OP_FORGET, OP_PAIR, OP_PASSKEY, OP_ANSWER, OP_SCAN] {
        assert_eq!(ask(&mut b, Shell, op, &[], 0).0, E_ACCES, "op {op}");
    }
    assert_eq!(b.pair_until, 0, "a refused PAIR opened nothing");
}

#[test]
fn anyone_else_is_refused_every_op_known_or_not() {
    let mut b = board();
    for op in OPS {
        assert!(!may(Other, op));
        assert_eq!(ask(&mut b, Other, op, &[], 0), (E_ACCES, vec![]), "op {op}");
    }
}

#[test]
fn settings_is_told_an_op_is_unknown() {
    assert_eq!(ask(&mut board(), Settings, 0x77, &[], 0).0, E_INVAL);
    assert_eq!(ask(&mut board(), Settings, 0, &[], 0).0, E_INVAL);
}
