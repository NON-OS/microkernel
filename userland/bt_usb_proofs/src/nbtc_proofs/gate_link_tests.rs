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

//! What the link tells NBTC and takes from it: a device that holds no
//! bond pairs only while the user lets one in, a bonded one reconnects
//! regardless, and a link whose bond is forgotten ends.

use crate::air::PEER;
use crate::bond_tests::{press, static_identity};
use crate::fixture::{air, keyboard, link};
use crate::fixture_link::plain;
use crate::link::{find_hid, End, FORGOTTEN};

#[test]
fn a_new_device_is_passed_over_while_pairing_is_shut() {
    let (mut host, _, mut env) = keyboard();
    env.user.may_pair = false;
    assert_eq!(find_hid(&mut host, &mut env, &plain()), Ok(None));
    assert!(env.now >= 10_000, "the look ran its whole time");
    assert_eq!(air(&mut host).seen.requests, 0);
}

#[test]
fn a_bonded_device_reconnects_with_pairing_shut() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(env.user.linked, [(1, PEER, true)], "pairing made it");
    env.user.may_pair = false;
    press(&mut host);
    let (end, _) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(env.user.linked[1], (1, PEER, false), "the kept bond resumed");
    assert_eq!(air(&mut host).seen.requests, 1, "paired once");
}

#[test]
fn a_forgotten_bond_ends_its_link() {
    let (mut host, ours, mut env) = keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    env.user.hang_up = true;
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp(FORGOTTEN));
    assert!(s.bond.is_some(), "it was the kept bond's link");
    assert!(!env.user.hang_up, "taken once");
}
