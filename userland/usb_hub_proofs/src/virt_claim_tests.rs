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
//! A claim on a virtual port, the slot its claimant's requests go to, and
//! what giving a slot back means to each of its two holders.

use nonos_usb_virt::{parse_park, Disable, Virt};

use crate::virt_park_tests::BODY;

fn parked() -> (Virt, u8) {
    let mut v = Virt::default();
    let vp = v.park(parse_park(&BODY).unwrap()).unwrap();
    (v, vp)
}

#[test]
fn the_first_claim_wins_and_its_slot_stands_for_the_parked_one() {
    let (mut v, vp) = parked();
    assert_eq!(v.claim(vp, 9).map(|p| p.slot), Some(5));
    assert_eq!(v.claim(vp, 10), None, "claimed already");
    assert_eq!((v.real(9), v.real(10), v.real(5)), (5, 10, 5));
    assert_eq!(v.claim(0x81, 11), None, "no such port");
    let (mut w, wp) = parked();
    assert_eq!(w.claim(wp, 5), None, "the parked slot cannot claim itself");
}

#[test]
fn a_child_that_leaves_unclaimed_goes_at_once() {
    let (mut v, vp) = parked();
    assert_eq!(v.disable(5), Disable::Detach(5));
    assert!(v.at(vp).is_none() && !v.knows(5));
    assert_eq!(v.disable(7), Disable::Plain, "not a slot of the table's");
}

#[test]
fn a_claimed_child_that_leaves_waits_for_its_claimant() {
    let (mut v, vp) = parked();
    v.claim(vp, 9).unwrap();
    assert_eq!(v.disable(5), Disable::Detached, "usb_hub0 saw it leave");
    let p = v.at(vp).unwrap();
    assert_eq!((p.connected, p.changed), (false, true), "read as a root-port detach");
    assert_eq!(v.claim(vp, 11), None, "nobody claims a child that left");
    assert_eq!(v.disable(9), Disable::ReleaseGone(5), "both slots go");
    assert!(v.at(vp).is_none());
}

#[test]
fn a_claimant_letting_go_parks_the_child_again() {
    let (mut v, vp) = parked();
    v.claim(vp, 9).unwrap();
    assert_eq!(v.disable(9), Disable::Release);
    assert_eq!((v.real(9), v.at(vp).unwrap().claimant), (9, None));
    assert_eq!(v.claim(vp, 12).map(|p| p.slot), Some(5), "claimable again");
}
