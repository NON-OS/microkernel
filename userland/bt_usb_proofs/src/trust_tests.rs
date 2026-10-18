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

//! Keys reach the input ring only from a bond the user's passkey
//! protected. A device that pairs by Just Works, which needs no one, may
//! move the pointer and click but never type, whatever it calls itself.

use crate::air::Act;
use crate::fixture::{air, link, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::method::IoCap;

const KEYS: u16 = 11;
const MOUSE: u16 = 16;

#[test]
fn a_keyboard_paired_by_just_works_cannot_type() {
    let peer = Peer::new(IoCap::NoInputNoOutput, false);
    let (mut host, ours, mut env) = with(peer, keyboard_mouse(), &HID_ADV);
    let script = [
        Act::Notify(KEYS, vec![0x08, 0, 0x15, 0, 0, 0, 0, 0]),
        Act::Notify(MOUSE, vec![0, 3, 0]),
        Act::Drop(0x13),
    ];
    air(&mut host).script.extend(script);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert!(!s.bond.unwrap().authenticated);
    assert_eq!(env.input, [Input::Motion { dx: 3, dy: 0 }], "a key got through");
    let warned = env.lines.iter().any(|l| l.starts_with("keys not posted"));
    assert!(warned, "{:?}", env.lines);
}
