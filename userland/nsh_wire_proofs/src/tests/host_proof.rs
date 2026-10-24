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

//! `HOST_PROOF`: the parts are placed, the enrolment inside it is judged
//! as a certificate, and the registry entries hold whole 45-byte records.

use crate::build::enrol::EnrolB;
use crate::build::proof::HostProofB;
use nsh_wire::{HostProof, WireError, ENTRY_LEN, MAX_DOC, MAX_ENTRIES};

fn parse(b: &HostProofB) -> Result<(), WireError> {
    HostProof::parse(&b.bytes()).map(|_| ())
}

#[test]
fn a_host_proof_places_its_parts() {
    let b = HostProofB::default();
    let bytes = b.bytes();
    let p = HostProof::parse(&bytes).expect("valid");
    assert_eq!((p.host.ed[0], p.host.ml[1951]), (0x4B, 0x4B));
    assert_eq!(p.enrol.host, b"db-1.eu.example");
    assert_eq!((p.doc, p.entries), (&b.doc[..], &b.entries[..]));
    assert_eq!(p.entries().count(), 2);
    assert_eq!(p.signed, &bytes[..bytes.len() - 3373]);
}

#[test]
fn a_bad_enrolment_inside_is_refused_as_the_certificate_would_be() {
    let enrol = EnrolB { version: 9, ..EnrolB::default() }.bytes();
    assert_eq!(parse(&HostProofB { enrol, ..HostProofB::default() }), Err(WireError::BodyVersion));
    assert_eq!(
        parse(&HostProofB { enrol: vec![], ..HostProofB::default() }),
        Err(WireError::Value)
    );
}

#[test]
fn the_document_is_one_byte_to_its_cap() {
    let d = HostProofB::default;
    assert_eq!(parse(&HostProofB { doc: vec![], ..d() }), Err(WireError::Value));
    assert_eq!(parse(&HostProofB { doc: vec![1; MAX_DOC], ..d() }), Ok(()));
    assert_eq!(parse(&HostProofB { doc: vec![1; MAX_DOC + 1], ..d() }), Err(WireError::Oversize));
}

#[test]
fn the_entries_are_whole_records_one_to_the_cap() {
    let d = HostProofB::default;
    for n in [0, 44, 46, 89] {
        assert_eq!(parse(&HostProofB { entries: vec![0; n], ..d() }), Err(WireError::Value), "{n}");
    }
    let full = vec![0; ENTRY_LEN * MAX_ENTRIES];
    assert_eq!(parse(&HostProofB { entries: full.clone(), ..d() }), Ok(()));
    let over = [full, vec![0; ENTRY_LEN]].concat();
    assert_eq!(parse(&HostProofB { entries: over, ..d() }), Err(WireError::Oversize));
}

#[test]
fn a_cut_or_padded_host_proof_is_refused() {
    let bytes = HostProofB::default().bytes();
    assert_eq!(HostProof::parse(&bytes[..bytes.len() - 1]), Err(WireError::Short));
    assert_eq!(HostProof::parse(&[bytes, vec![0]].concat()), Err(WireError::Trailing));
}
