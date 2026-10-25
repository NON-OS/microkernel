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

//! The enrolment certificate: every field is placed where the
//! specification puts it, and each malformed field is refused.

use crate::build::enrol::EnrolB;
use nsh_wire::{Enrol, WireError};

fn parse(b: &EnrolB) -> Result<(), WireError> {
    Enrol::parse(&b.bytes()).map(|_| ())
}

#[test]
fn a_certificate_places_every_field() {
    let b = EnrolB::default();
    let bytes = b.bytes();
    let e = Enrol::parse(&bytes).expect("valid");
    assert_eq!((e.fleet_id, e.host, e.ak_public), (&b.fleet_id, &b.host[..], &b.ak));
    assert_eq!((e.host_keys, e.not_after_ms, e.authority), (&b.host_keys, 9_000, &b.authority));
    assert_eq!(e.pcrs.map(|p| p[0]), [0, 1, 2, 7]);
    assert_eq!(e.boot_applications().collect::<Vec<_>>(), [&[0xB0; 32]]);
    // After PCR 7 and before not_after_ms: preamble, fleet, host, AK, keys, PCRs.
    let at = 8 + 16 + 1 + b.host.len() + 64 + 64 + 4 * 32;
    assert_eq!((&bytes[at..at + 2], &bytes[at + 2..at + 34]), (&[1, 1][..], &[0xB0; 32][..]));
    assert_eq!((e.sig.ed[0], e.sig.ml[3308]), (0xE7, 0xE7));
    assert_eq!(e.signed, &bytes[..bytes.len() - 3373]);
}

#[test]
fn the_magic_version_and_reserved_field_are_judged() {
    assert_eq!(parse(&EnrolB { magic: *b"NSHG", ..EnrolB::default() }), Err(WireError::Magic));
    assert_eq!(parse(&EnrolB { version: 2, ..EnrolB::default() }), Err(WireError::BodyVersion));
    assert_eq!(parse(&EnrolB { reserved: 1, ..EnrolB::default() }), Err(WireError::Reserved));
}

#[test]
fn a_zero_fleet_and_a_bad_host_name_are_refused() {
    assert_eq!(parse(&EnrolB { fleet_id: [0; 16], ..EnrolB::default() }), Err(WireError::Value));
    let mut one = [0; 16];
    one[15] = 1;
    assert_eq!(parse(&EnrolB { fleet_id: one, ..EnrolB::default() }), Ok(()));
    for host in [&b"DB.example"[..], b"db..x", b""] {
        let b = EnrolB { host: host.to_vec(), ..EnrolB::default() };
        assert_eq!(parse(&b), Err(WireError::Value), "{host:?}");
    }
}

#[test]
fn a_cut_or_padded_certificate_is_refused() {
    let bytes = EnrolB::default().bytes();
    assert_eq!(Enrol::parse(&bytes[..bytes.len() - 1]), Err(WireError::Short));
    assert_eq!(Enrol::parse(&[bytes.clone(), vec![0]].concat()), Err(WireError::Trailing));
}
