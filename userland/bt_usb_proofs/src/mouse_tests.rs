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

//! A mouse pairs by Just Works, with nothing shown; its buttons and motion
//! reach the input ring, and whatever is held when the link drops is
//! released.

use crate::air::Act;
use crate::fixture::{keyboard, link, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::method::IoCap;

/// The boot keyboard and boot mouse input reports' value handles in
/// `keyboard_mouse`.
const KEYS: u16 = 11;
const MOUSE: u16 = 16;

#[test]
fn a_mouse_pairs_by_just_works_and_its_input_is_posted() {
    let peer = Peer::new(IoCap::NoInputNoOutput, false);
    let (mut host, ours, mut env) = with(peer, keyboard_mouse(), &HID_ADV);
    let script = [Act::Notify(MOUSE, vec![0x01, 5, 0xfb]), Act::Drop(0x08)];
    crate::fixture::air(&mut host).script.extend(script);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x08));
    // The HID host posts a report's motion ahead of its buttons.
    let want = [Input::Motion { dx: 5, dy: -5 }, Input::ButtonDown(1), Input::ButtonUp(1)];
    assert_eq!(env.input, want);
    assert!(env.passkeys.is_empty(), "Just Works shows nothing");
    assert!(!s.bond.unwrap().authenticated, "Just Works is not protected");
}

#[test]
fn a_key_held_when_the_link_drops_is_released() {
    let (mut host, ours, mut env) = keyboard();
    let script = [Act::Notify(KEYS, vec![0x02, 0, 0x05, 0, 0, 0, 0, 0]), Act::Drop(0x08)];
    crate::fixture::air(&mut host).script.extend(script);
    link(&mut host, &ours, &mut env).unwrap();
    // Left Shift and B down; the HID host releases the modifier first.
    let want = [Input::KeyDown(0xe1), Input::KeyDown(0x05), Input::KeyUp(0xe1), Input::KeyUp(0x05)];
    assert_eq!(env.input, want);
}
