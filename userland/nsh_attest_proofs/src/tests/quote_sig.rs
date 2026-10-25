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

//! Step 3, the signature: the document's AK signed exactly these bytes.

use nsh_attest::Refusal;

use super::{passes, refused};
use crate::build::sig::tpmt;
use crate::build::Spec;

#[test]
fn a_quote_signed_by_another_key_is_refused() {
    assert_eq!(refused(Spec { signing_key: 2, ..Spec::default() }), Refusal::Signature);
}

#[test]
fn a_document_naming_another_ak_is_refused() {
    assert_eq!(refused(Spec { doc_key: 2, ..Spec::default() }), Refusal::Signature);
}

#[test]
fn a_signed_byte_changed_after_signing_is_refused() {
    // In clockInfo, which no other check reads.
    assert_eq!(refused(Spec { after_sign: |a| a[80] ^= 1, ..Spec::default() }), Refusal::Signature);
}

#[test]
fn only_ecdsa_over_sha256_is_accepted() {
    let rsa = |r: [u8; 32], s: [u8; 32]| tpmt(0x0014, 0x000B, &r, &s);
    assert_eq!(refused(Spec { sig: rsa, ..Spec::default() }), Refusal::Signature);
    let sha384 = |r: [u8; 32], s: [u8; 32]| tpmt(0x0018, 0x000C, &r, &s);
    assert_eq!(refused(Spec { sig: sha384, ..Spec::default() }), Refusal::Signature);
}

#[test]
fn r_and_s_swapped_are_refused() {
    let swap = |r: [u8; 32], s: [u8; 32]| tpmt(0x0018, 0x000B, &s, &r);
    assert_eq!(refused(Spec { sig: swap, ..Spec::default() }), Refusal::Signature);
    passes(Spec::default());
}

#[test]
fn a_signer_that_is_not_a_sha256_name_is_refused() {
    assert_eq!(refused(Spec { quote: |q| q.signer.clear(), ..Spec::default() }), Refusal::Signer);
    let sha1 = |q: &mut crate::build::attest::Quote| q.signer[1] = 0x04;
    assert_eq!(refused(Spec { quote: sha1, ..Spec::default() }), Refusal::Signer);
    let long = |q: &mut crate::build::attest::Quote| q.signer.extend([0; 32]);
    assert_eq!(refused(Spec { quote: long, ..Spec::default() }), Refusal::Signer);
}
