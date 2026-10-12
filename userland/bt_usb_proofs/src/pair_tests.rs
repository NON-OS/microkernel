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

//! One LE Advertising Report event may carry several reports (Vol 4,
//! Part E, 7.7.65.2): a bonded peer among them is taken whatever its
//! place, and otherwise the first new HID advertiser.

use crate::air::PEER;
use crate::bond_tests::{press, static_identity};
use crate::fixture::{air, keyboard, link, HID_ADV};
use crate::link::find_hid;
use crate::reconnect_tests::STRANGER;

const OTHER: [u8; 6] = [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0xca];

/// One event with an ADV_IND from each address, random addresses, in
/// order, RSSI -60.
fn together(from: &[[u8; 6]], data: &[u8]) -> Vec<u8> {
    let mut sub = vec![0x02, from.len() as u8];
    for a in from {
        sub.extend_from_slice(&[0x00, 0x01]);
        sub.extend_from_slice(a);
        sub.push(data.len() as u8);
        sub.extend_from_slice(data);
        sub.push(-60i8 as u8);
    }
    [vec![0x3e, sub.len() as u8], sub].concat()
}

#[test]
fn of_two_new_advertisers_in_one_event_the_first_is_taken() {
    let (mut host, _, mut env) = keyboard();
    air(&mut host).others.push(together(&[STRANGER, OTHER], &HID_ADV));
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    assert_eq!(peer.addr.0, STRANGER);
}

#[test]
fn a_bonded_peer_after_a_new_one_in_the_same_event_is_taken() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    air(&mut host).others.push(together(&[STRANGER, PEER], &HID_ADV));
    // The peer's own report comes later: make it one the look ignores.
    air(&mut host).adv_kind = 0x03;
    let start = env.now;
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    assert_eq!(peer.addr.0, PEER);
    assert!(env.now - start < 2_000, "the bonded peer waited");
}
