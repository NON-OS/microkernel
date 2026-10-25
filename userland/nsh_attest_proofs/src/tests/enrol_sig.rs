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

//! Step 4, the signature: a certificate the policy's authority signed with
//! both halves of its hybrid key, and that parses.

use nsh_attest::{Evidence, EvidenceError, Refusal};
use nsh_crypto::Transcript;

use super::refused;
use crate::build::keys::STRANGER;
use crate::build::Spec;

#[test]
fn a_certificate_that_does_not_parse_gives_no_evidence() {
    // nsh_wire reads the certificate inside HOST_PROOF, so this is refused
    // before any check, as PROTOCOL; `EnrolFormat` stays as a second wall.
    for edit in [|b: &mut Vec<u8>| b.push(0), |b: &mut Vec<u8>| b.truncate(100)] {
        let b = Spec { cert_bytes: edit, ..Spec::default() }.build();
        let r = Evidence::verify(&b.proof(), &Transcript::default()).map(|_| ());
        assert!(matches!(r, Err(EvidenceError::Wire(_))));
    }
}

#[test]
fn a_certificate_from_an_authority_the_policy_does_not_hold_is_refused() {
    let s = Spec { cert: |c| c.authority_id = [0xEE; 32], ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolAuthority);
}

#[test]
fn a_certificate_signed_by_another_key_is_refused() {
    let s = Spec { cert_signer: STRANGER, ..Spec::default() };
    assert_eq!(refused(s), Refusal::EnrolSignature);
}
