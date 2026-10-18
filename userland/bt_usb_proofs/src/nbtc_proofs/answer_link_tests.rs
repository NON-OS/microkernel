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

//! Pairing answered by the user through NBTC: digits typed here for a
//! device that shows them, and numbers confirmed on both sides (Vol 3,
//! Part H, 2.3.5.1). Either is authenticated, so the keyboard's keys post.

use crate::bond_tests::press;
use crate::env::TestEnv;
use crate::fixture::{air, link, with, Stack, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::link::{Answer, Asked, End};
use crate::smp_sim::Peer;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::method::IoCap;
use nonos_bt_wire::addr::BdAddr;

pub const SHOWN: u32 = 412_907;

pub fn peer(io_cap: IoCap) -> (Stack, BdAddr, TestEnv) {
    with(Peer::new(io_cap, true), keyboard_mouse(), &HID_ADV)
}

#[test]
fn digits_the_device_shows_typed_here_pair_it() {
    let (mut host, ours, mut env) = peer(IoCap::DisplayOnly);
    env.user.answers.push_back(Answer::Passkey(SHOWN));
    press(&mut host);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(env.user.asked, [Asked::Passkey]);
    assert!(s.bond.unwrap().authenticated);
    assert_eq!(env.user.ready, [(true, true, true)]);
    assert_eq!(env.input, [Input::KeyDown(4), Input::KeyUp(4)]);
}

#[test]
fn numbers_confirmed_the_same_pair_the_device() {
    let (mut host, ours, mut env) = peer(IoCap::DisplayYesNo);
    env.user.answers.push_back(Answer::Same(true));
    press(&mut host);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(env.user.asked, [Asked::Number(air(&mut host).smp.number())], "the peer's number");
    assert!(s.bond.unwrap().authenticated);
    assert_eq!(env.input, [Input::KeyDown(4), Input::KeyUp(4)]);
}
