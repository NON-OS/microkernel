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

//! The enrolment's boot applications: the section's own version, the cap
//! of eight, and no zero or repeated digest. The largest certificate holds
//! a full host name and every boot application.

use crate::build::enrol::EnrolB;
use nsh_wire::{Enrol, WireError, MAX_BOOT_APPS, MAX_ENROL};

fn parse(b: &EnrolB) -> Result<(), WireError> {
    Enrol::parse(&b.bytes()).map(|_| ())
}

fn apps(n: u8) -> Vec<[u8; 32]> {
    (1..=n).map(|i| [i; 32]).collect()
}

#[test]
fn none_and_up_to_eight_are_read_in_order_and_asked_one_by_one() {
    assert_eq!(parse(&EnrolB { boot_apps: vec![], ..EnrolB::default() }), Ok(()));
    let b = EnrolB { boot_apps: apps(8), ..EnrolB::default() };
    let bytes = b.bytes();
    let e = Enrol::parse(&bytes).expect("valid");
    let read: Vec<[u8; 32]> = e.boot_applications().copied().collect();
    assert_eq!(read, apps(8));
    assert!((1..=8).all(|i| e.boot_application(&[i; 32])));
    assert!(!e.boot_application(&[9; 32]) && !e.boot_application(&[0; 32]));
    assert_eq!(MAX_BOOT_APPS, 8);
}

#[test]
fn a_ninth_a_zero_or_a_repeated_application_is_refused() {
    assert_eq!(parse(&EnrolB { boot_apps: apps(9), ..EnrolB::default() }), Err(WireError::Value));
    for bad in [vec![[0; 32]], vec![[1; 32], [0; 32]], vec![[1; 32], [2; 32], [1; 32]]] {
        let b = EnrolB { boot_apps: bad.clone(), ..EnrolB::default() };
        assert_eq!(parse(&b), Err(WireError::Value), "{bad:?}");
    }
    let mut one_byte = [0; 32];
    one_byte[31] = 1;
    assert_eq!(parse(&EnrolB { boot_apps: vec![one_byte], ..EnrolB::default() }), Ok(()));
}

#[test]
fn a_section_version_this_client_does_not_know_is_refused() {
    for v in [0, 2, 0xFF] {
        let b = EnrolB { boot_version: v, ..EnrolB::default() };
        assert_eq!(parse(&b), Err(WireError::BodyVersion), "{v}");
    }
}

#[test]
fn the_largest_certificate_is_max_enrol() {
    let b = EnrolB {
        host: [&[b'a'; 63][..], b".", &[b'b'; 63], b".", &[b'c'; 63], b".", &[b'd'; 61]].concat(),
        boot_apps: apps(8),
        ..EnrolB::default()
    };
    assert_eq!(b.host.len(), 253);
    assert_eq!(b.bytes().len(), MAX_ENROL);
    assert_eq!(parse(&b), Ok(()));
}
