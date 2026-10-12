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

//! Which advertiser a look takes once bonds are kept: a bonded peer at
//! once, from directed advertising too (Vol 6, Part B, 2.3.1.2), and a
//! new one only after it has waited, so a stranger never wins a race
//! against a bonded keyboard.

use crate::air::{advertised, PEER};
use crate::bond_tests::{press, static_identity};
use crate::fixture::{air, keyboard, link, HID_ADV};
use crate::link::{find_hid, End};
use nonos_bt_hogp::report::Input;

pub const STRANGER: [u8; 6] = [0x01, 0x02, 0x03, 0x04, 0x05, 0xc9];

#[test]
fn a_bonded_keyboard_advertising_directed_here_is_taken_and_resumed() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    // ADV_DIRECT_IND: the advertiser's and the target's addresses only.
    (air(&mut host).adv_kind, air(&mut host).adv) = (0x01, Vec::new());
    press(&mut host);
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(air(&mut host).seen.requests, 1, "paired again");
    assert!(env.said("encrypted with the kept bond"), "{:?}", env.lines);
    let typed = [Input::KeyDown(4), Input::KeyUp(4)];
    assert_eq!(env.input, [typed, typed].concat());
}

#[test]
fn a_bonded_peer_that_may_not_be_connected_to_is_passed_over() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    // ADV_NONCONN_IND, with the HID service named all the same.
    air(&mut host).adv_kind = 0x03;
    assert_eq!(find_hid(&mut host, &mut env), Ok(None));
}

#[test]
fn a_bonded_peer_is_taken_over_a_stranger_heard_first() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    air(&mut host).others.push(advertised(STRANGER, 0x00, &HID_ADV));
    // What the bonded peer advertises does not matter: here, no HID.
    air(&mut host).adv = vec![2, 0x01, 0x06, 3, 0x03, 0x0f, 0x18];
    let start = env.now;
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    assert_eq!(peer.addr.0, PEER);
    assert!(env.now - start < 2_000, "the bonded peer waited");
}
