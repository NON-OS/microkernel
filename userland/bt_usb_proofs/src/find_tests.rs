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

//! The passive scan: it stops before a connection is asked for, and a
//! look with no HID device in range ends on time with the radio quiet.

use crate::fixture::{air, keyboard, with};
use crate::hogp_sim::keyboard_mouse;
use crate::link::find_hid;
use crate::smp_sim::Peer;
use nonos_bt_smp::method::IoCap;
use nonos_bt_wire::hci::opcode as op;

fn opcodes(commands: &[Vec<u8>]) -> Vec<u16> {
    commands.iter().map(|c| u16::from_le_bytes([c[0], c[1]])).collect()
}

#[test]
fn no_hid_device_ends_the_look_with_the_scan_stopped() {
    let battery_only = [2, 0x01, 0x06, 3, 0x03, 0x0f, 0x18];
    let peer = Peer::new(IoCap::NoInputNoOutput, false);
    let (mut host, _, mut env) = with(peer, keyboard_mouse(), &battery_only);
    assert_eq!(find_hid(&mut host, &mut env), Ok(None));
    assert!(env.now >= 10_000, "the look ended early, at {} ms", env.now);
    let last = air(&mut host).commands.last().unwrap().clone();
    assert_eq!(last, [0x0c, 0x20, 2, 0, 0], "the scan was left on");
    // The scan was passive: LE Set Scan Parameters' type octet is 0.
    let scan = air(&mut host).commands.iter().find(|c| c[..2] == [0x0b, 0x20]).unwrap().clone();
    assert_eq!(scan[3], 0x00);
}

#[test]
fn the_scan_stops_before_the_connection_is_asked_for() {
    let (mut host, _, mut env) = keyboard();
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    assert_eq!(peer.addr.0, crate::air::PEER);
    let ops = opcodes(&air(&mut host).commands);
    let enable = op::LE_SET_SCAN_ENABLE.0;
    let n = ops.iter().filter(|&&o| o == enable).count();
    assert_eq!(n, 2, "scan on, then off: {ops:04x?}");
    assert_eq!(*ops.last().unwrap(), enable);
}
