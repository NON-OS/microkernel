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

//! A whole link: heard, connected, paired by passkey entry, encrypted,
//! the HID service set up, a key pressed and released, and the link lost.

use crate::air::Act;
use crate::fixture::{air, keyboard, link};
use crate::link::End;
use nonos_bt_hogp::report::Input;

/// The boot keyboard input report's value handle in `keyboard_mouse`.
const KEYS: u16 = 11;

#[test]
fn a_keyboard_pairs_and_its_keys_reach_the_input_ring() {
    let (mut host, ours, mut env) = keyboard();
    let script = [
        Act::Notify(KEYS, vec![0, 0, 0x04, 0, 0, 0, 0, 0]),
        Act::Notify(KEYS, vec![0; 8]),
        Act::Drop(0x13),
    ];
    air(&mut host).script.extend(script);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(env.input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    // Passkey entry with our display: one passkey shown, and the bond
    // marked as protected from a device in the middle.
    assert_eq!(env.passkeys.len(), 1);
    let bond = s.bond.expect("no bond");
    assert!(bond.authenticated);
    assert_eq!(Some(bond.ltk), air(&mut host).ltk);
    assert!(env.lines.iter().any(|l| l == "HID ready: keyboard mouse"), "{:?}", env.lines);
}

#[test]
fn hid_services_past_the_first_four_are_left_with_one_line() {
    use crate::fixture::{with, HID_ADV};
    use crate::hogp_sim::devices::{NOTIFY, READ, WWR};
    use crate::hogp_sim::Db;
    use crate::smp_sim::Peer;
    use nonos_bt_smp::method::IoCap;
    let boot = |db: Db| {
        db.service(0x1812)
            .char(READ | WWR, 0x2a4e, &[1])
            .char(READ | NOTIFY, 0x2a22, &[0; 8])
            .desc(0x2902, &[0, 0])
    };
    let db = (0..6).fold(Db::default(), |db, _| boot(db));
    let (mut host, ours, mut env) = with(Peer::new(IoCap::KeyboardOnly, true), db, &HID_ADV);
    air(&mut host).script.push_back(Act::Drop(0x13));
    link(&mut host, &ours, &mut env).unwrap();
    let past =
        env.lines.iter().filter(|l| *l == "HID services past the first four left unread").count();
    assert_eq!(past, 1, "{:?}", env.lines);
}
