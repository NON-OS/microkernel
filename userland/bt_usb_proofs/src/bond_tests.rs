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

//! A bonded peer comes back: found by its identity, encrypted with the
//! kept key (Vol 3, Part H, 2.4.4), and never paired again.

use crate::air::{Act, PEER};
use crate::fixture::{air, keyboard, link, Stack};
use crate::link::{hang_up, End};
use nonos_bt_hogp::report::Input;

pub const KEYS: u16 = 11;

/// The peer's identity is the static address it advertises from.
pub fn static_identity(host: &mut Stack) {
    air(host).smp.identity = [&[1u8][..], &PEER].concat().try_into().unwrap();
}

pub fn press(host: &mut Stack) {
    let down = Act::Notify(KEYS, vec![0, 0, 0x04, 0, 0, 0, 0, 0]);
    air(host).script.extend([down, Act::Notify(KEYS, vec![0; 8]), Act::Drop(0x13)]);
}

#[test]
fn a_bonded_keyboard_is_encrypted_with_its_kept_key_and_types() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(env.bonds.book().bonds().len(), 1);
    press(&mut host);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(air(&mut host).seen.requests, 1, "paired again");
    assert_eq!(env.passkeys.len(), 1);
    assert!(env.said("encrypted with the kept bond"), "{:?}", env.lines);
    assert!(s.bond.unwrap().authenticated);
    let typed = [Input::KeyDown(4), Input::KeyUp(4)];
    assert_eq!(env.input, [typed, typed].concat());
}

#[test]
fn a_peer_that_dropped_the_bond_is_forgotten_and_paired_afresh() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    // The peer was reset: it holds no key for this host any more.
    air(&mut host).smp.ltk = [0x5a; 16];
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("the peer no longer holds the bond"));
    assert!(env.bonds.book().bonds().is_empty());
    hang_up(&mut host, s.handle).unwrap();
    air(&mut host).script.clear();
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(air(&mut host).seen.requests, 2);
    assert_eq!(env.bonds.book().bonds().len(), 1);
}
