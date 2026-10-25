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

//! Evidence comes only from a `HOST_PROOF` whose two host signatures both
//! verify over this transcript; a body nsh_wire refuses is refused alike.

use nsh_attest::{Evidence, EvidenceError};
use nsh_crypto::Transcript;
use nsh_wire::{Kind, WireError};

use crate::build::proof::host_proof;
use crate::build::Spec;

fn verify(body: &[u8]) -> Result<(), EvidenceError> {
    Evidence::verify(body, &Transcript::default()).map(|_| ())
}

#[test]
fn a_signed_host_proof_gives_the_parts_the_check_reads() {
    let b = Spec::default().build();
    let body = b.proof();
    let ev = Evidence::verify(&body, &Transcript::default()).expect("verifies");
    assert_eq!((ev.doc(), ev.enrol(), ev.entries()), (&b.doc[..], &b.enrol[..], &b.entries[..]));
    assert!(nsh_attest::check(&ev, &b.session(), &b.policy()).is_ok());
}

#[test]
fn a_bad_ed25519_half_alone_is_refused() {
    let mut body = Spec::default().build().proof();
    let n = body.len();
    body[n - 3309 - 1] ^= 1;
    assert_eq!(verify(&body), Err(EvidenceError::Signature));
}

#[test]
fn a_bad_ml_dsa_half_alone_is_refused() {
    let mut body = Spec::default().build().proof();
    let n = body.len();
    body[n - 1500] ^= 1;
    assert_eq!(verify(&body), Err(EvidenceError::Signature));
}

#[test]
fn a_proof_signed_over_another_transcript_or_changed_is_refused() {
    let b = Spec::default().build();
    let mut t = Transcript::default();
    t.push(Kind::ServerHello, b"another hello");
    let signed = host_proof(&b, &t);
    assert_eq!(verify(&signed), Err(EvidenceError::Signature));
    assert!(Evidence::verify(&signed, &t).is_ok(), "over the transcript it was signed on");
    let mut body = b.proof();
    body[2100] ^= 1;
    assert_eq!(verify(&body), Err(EvidenceError::Signature));
}

#[test]
fn a_host_proof_nsh_wire_refuses_is_refused() {
    let mut body = Spec::default().build().proof();
    body.push(0);
    assert_eq!(verify(&body), Err(EvidenceError::Wire(WireError::Trailing)));
    assert_eq!(verify(&body[..100]), Err(EvidenceError::Wire(WireError::Short)));
}
