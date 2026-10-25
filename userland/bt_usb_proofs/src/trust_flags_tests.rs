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

//! E-61b: the G-76 trust an input event carries. Every event the capsule
//! posts is stamped SOURCE_BT; BOND_AUTHENTICATED exactly when its bond was
//! authenticated, carried through a reconnect; a Just Works bond never gets
//! it. The flags are read from the live session bond, not a stale record.

use crate::air::Act;
use crate::bond_tests::{press, static_identity};
use crate::fixture::{air, keyboard, link, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::link::trust::{source_flags, BOND_AUTHENTICATED, SOURCE_BT};
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_smp::method::IoCap;

#[test]
fn source_flags_sets_bt_always_and_authenticated_only_when_asked() {
    assert_eq!(source_flags(true), SOURCE_BT | BOND_AUTHENTICATED);
    assert_eq!(source_flags(false), SOURCE_BT);
    for a in [false, true] {
        assert_ne!(source_flags(a) & SOURCE_BT, 0, "SOURCE_BT is always set");
        assert_eq!(
            source_flags(a) & BOND_AUTHENTICATED != 0,
            a,
            "BOND_AUTHENTICATED tracks the bond"
        );
    }
}

#[test]
fn an_authenticated_keyboard_stamps_both_bits_through_a_reconnect() {
    let (mut host, ours, mut env) = keyboard(); // KeyboardOnly + MITM: passkey
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    press(&mut host);
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert!(!env.input.is_empty(), "the keyboard typed");
    assert_eq!(env.input.len(), env.input_flags.len());
    assert!(
        env.input_flags.iter().all(|&f| f == SOURCE_BT | BOND_AUTHENTICATED),
        "every key, across the reconnect, is authenticated: {:?}",
        env.input_flags
    );
}

#[test]
fn a_just_works_mouse_stamps_bt_without_authenticated() {
    let peer = Peer::new(IoCap::NoInputNoOutput, false); // Just Works
    let (mut host, ours, mut env) = with(peer, keyboard_mouse(), &HID_ADV);
    air(&mut host).script.extend([Act::Notify(16, vec![0x01, 5, 0xfb]), Act::Drop(0x08)]);
    link(&mut host, &ours, &mut env).unwrap();
    assert!(!env.input.is_empty(), "the mouse moved");
    assert!(
        env.input_flags.iter().all(|&f| f == SOURCE_BT),
        "a Just Works device is BT but never authenticated: {:?}",
        env.input_flags
    );
}
