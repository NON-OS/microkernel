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

//! What a kept bond carries back: only PIN or Key Missing forgets it, a
//! peer behind a new private address is still found, and a bond Just Works
//! made still may not type.

use crate::air::Act;
use crate::bond_tests::{press, static_identity, KEYS};
use crate::fixture::{air, keyboard, link, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::privacy::{prand, rpa};

#[test]
fn a_refusal_for_another_reason_keeps_the_bond() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    air(&mut host).cfg.fail = Some((0x2019, 0x0c));
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("encryption with the kept bond failed"));
    assert_eq!(env.bonds.book().bonds().len(), 1);
}

#[test]
fn a_bonded_peer_is_found_behind_a_new_private_address() {
    let (mut host, ours, mut env) = keyboard();
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    let irk = env.bonds.book().bonds()[0].irk.expect("no IRK kept");
    air(&mut host).seen.from = rpa(&irk, &prand([0x12, 0x34, 0x56]).unwrap()).0;
    press(&mut host);
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(air(&mut host).seen.requests, 1, "paired again");
    assert_eq!(env.input.len(), 4);
}

#[test]
fn a_just_works_bond_still_cannot_type_when_it_comes_back() {
    let peer = Peer::new(IoCap::NoInputNoOutput, false);
    let (mut host, ours, mut env) = with(peer, keyboard_mouse(), &HID_ADV);
    static_identity(&mut host);
    for _ in 0..2 {
        let down = Act::Notify(KEYS, vec![0, 0, 0x04, 0, 0, 0, 0, 0]);
        air(&mut host).script.extend([down, Act::Drop(0x13)]);
        let (_, s) = link(&mut host, &ours, &mut env).unwrap();
        assert!(!s.bond.unwrap().authenticated);
    }
    assert_eq!(air(&mut host).seen.requests, 1);
    assert_eq!(env.input, Vec::<Input>::new());
}
