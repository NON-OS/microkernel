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

//! How long a new advertiser waits: not at all while no bond is kept, and
//! HOLD_MS from when it was first heard once one is, so a bonded device
//! has the time to be heard first.

use crate::air::advertised;
use crate::fixture::{air, keyboard, HID_ADV};
use crate::link::find_hid;
use crate::reconnect_tests::STRANGER;
use crate::ring_tests::record;

#[test]
fn with_bonds_kept_the_first_new_advertiser_is_taken_after_its_wait() {
    let (mut host, _, mut env) = keyboard();
    env.bonds.keep(record(1)).unwrap();
    air(&mut host).others.push(advertised(STRANGER, 0x00, &HID_ADV));
    let start = env.now;
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    // The stranger was heard first; the peer heard after it, also new,
    // neither replaces it nor starts its wait again.
    assert_eq!(peer.addr.0, STRANGER);
    let waited = env.now - start;
    assert!((2_000..2_100).contains(&waited), "taken after {waited} ms");
}

#[test]
fn with_no_bond_kept_a_new_advertiser_is_taken_at_once() {
    let (mut host, _, mut env) = keyboard();
    air(&mut host).others.push(advertised(STRANGER, 0x00, &HID_ADV));
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    assert_eq!(peer.addr.0, STRANGER);
    assert!(env.now < 100, "a first pairing waited {} ms", env.now);
}

#[test]
fn a_new_advertiser_heard_late_is_taken_when_the_look_ends() {
    let (mut host, _, mut env) = keyboard();
    env.bonds.keep(record(1)).unwrap();
    // Each report comes in two transfers, each read at a millisecond of
    // the proofs' clock, so the stranger is first heard about 8.5 s in.
    let battery = [2, 0x01, 0x06, 3, 0x03, 0x0f, 0x18];
    let noise = advertised(STRANGER, 0x00, &battery);
    air(&mut host).others.extend(std::iter::repeat_n(noise, 4_250));
    air(&mut host).others.push(advertised(STRANGER, 0x00, &HID_ADV));
    let peer = find_hid(&mut host, &mut env).unwrap().unwrap();
    assert_eq!(peer.addr.0, STRANGER);
    assert!((10_000..10_100).contains(&env.now), "taken at {} ms", env.now);
}

#[test]
fn directed_advertising_from_a_peer_with_no_bond_is_passed_over() {
    let (mut host, _, mut env) = keyboard();
    (air(&mut host).adv_kind, air(&mut host).adv) = (0x01, Vec::new());
    assert_eq!(find_hid(&mut host, &mut env), Ok(None));
    assert!(env.now >= 10_000, "the look ended early, at {} ms", env.now);
}
