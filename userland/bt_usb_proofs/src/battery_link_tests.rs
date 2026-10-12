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

//! The HID device's battery over a whole link: read once its reports are
//! on, then each notified level, given to the capsule as it comes. A level
//! is not input, so a Just Works link gives it as well.

use crate::air::Act;
use crate::fixture::{air, keyboard, link, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::smp_sim::Peer;
use nonos_bt_hogp::host::Battery;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::method::IoCap;

/// The boot keyboard input report's and the Battery Level's value handles
/// in `keyboard_mouse`.
const KEYS: u16 = 11;
const LEVEL: u16 = 23;

const fn at(percent: u8) -> Battery {
    Battery { instance: 0, percent }
}

#[test]
fn a_keyboard_s_level_is_read_then_follows_its_notifications() {
    let (mut host, ours, mut env) = keyboard();
    let script = [
        Act::Notify(LEVEL, vec![15]),
        Act::Notify(KEYS, vec![0, 0, 0x04, 0, 0, 0, 0, 0]),
        Act::Notify(LEVEL, vec![14]),
        Act::Drop(0x13),
    ];
    air(&mut host).script.extend(script);
    link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(env.batteries, [at(90), at(15), at(14)]);
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
}

#[test]
fn a_just_works_mouse_gives_its_level_and_nothing_out_of_range() {
    let peer = Peer::new(IoCap::NoInputNoOutput, false);
    let (mut host, ours, mut env) = with(peer, keyboard_mouse(), &HID_ADV);
    let script = [Act::Notify(LEVEL, vec![101]), Act::Notify(LEVEL, vec![3]), Act::Drop(0x08)];
    air(&mut host).script.extend(script);
    let (_, s) = link(&mut host, &ours, &mut env).unwrap();
    assert!(!s.bond.unwrap().authenticated);
    assert_eq!(env.batteries, [at(90), at(3)]);
}
