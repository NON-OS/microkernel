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

//! LE Set Random Address (Vol 4, Part E, 7.8.4): sent, refused, then used.

use crate::sim::config::Config;
use crate::sim::events::status;
use crate::sim::link_events::{connected, CONNECT};
use crate::sim::stack::stack;
use nonos_bt_hci::host::DEFAULT_SCAN;
use nonos_bt_hci::HciError;
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::WireError;

/// The resolvable private address of Vol 3, Part H, D.7.
const RPA: BdAddr = BdAddr([0xaa, 0xfb, 0x0d, 0x94, 0x81, 0x70]);

#[test]
fn the_address_goes_out_and_scans_and_connections_use_it() {
    let create = op::LE_CREATE_CONNECTION.0;
    let mut s =
        stack(Config::scripted(vec![(create, vec![status(0, 1, create), connected(0, 1)])]));
    assert_eq!(s.own_address(), AddrType::Public);
    s.use_random_address(RPA).unwrap();
    s.scan_passive(DEFAULT_SCAN).unwrap();
    s.connect(&CONNECT).unwrap();
    assert_eq!(s.own_address(), AddrType::Random);
    let cmds = &s.transport().pipes().commands;
    assert_eq!(cmds[0].1, [0x05, 0x20, 6, 0xaa, 0xfb, 0x0d, 0x94, 0x81, 0x70]);
    assert_eq!(cmds[1].1[3 + 5], 0x01);
    assert_eq!(cmds[3].1[3 + 12], 0x01);
}

#[test]
fn reserved_and_forbidden_addresses_are_refused_unsent() {
    let mut s = stack(Config::default());
    let bad = [
        BdAddr([1, 2, 3, 4, 5, 0x80]),
        BdAddr([0; 6]),
        BdAddr([0, 0, 0, 0, 0, 0xc0]),
        BdAddr([0xff; 6]),
        BdAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0x3f]),
        BdAddr([1, 2, 3, 0, 0, 0x40]),
        BdAddr([1, 2, 3, 0xff, 0xff, 0x7f]),
    ];
    for a in bad {
        assert_eq!(s.use_random_address(a), Err(HciError::Wire(WireError::Reserved)), "{a:?}");
    }
    assert!(s.transport().pipes().commands.is_empty());
    s.use_random_address(BdAddr([1, 2, 3, 4, 5, 0xc6])).unwrap();
}

#[test]
fn an_address_the_controller_refuses_leaves_the_public_one() {
    let set = op::LE_SET_RANDOM_ADDRESS;
    let mut s = stack(Config { fail: Some((set.0, 0x0c)), ..Config::default() });
    assert_eq!(s.use_random_address(RPA), Err(HciError::Status(set, 0x0c)));
    assert_eq!(s.own_address(), AddrType::Public);
    s.scan_passive(DEFAULT_SCAN).unwrap();
    assert_eq!(s.transport().pipes().commands[1].1[3 + 5], 0x00);
}
