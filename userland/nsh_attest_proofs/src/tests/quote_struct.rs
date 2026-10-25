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

//! Step 3, the structure: a TPM quote, read to its last byte, carrying the
//! client's own `qualifying_data` as `extraData`.

use nsh_attest::{QuoteFault, Refusal};

use super::refused;
use crate::build::Spec;

const MALFORMED: Refusal = Refusal::Quote(QuoteFault::Malformed);

#[test]
fn a_structure_the_tpm_did_not_generate_is_refused() {
    let s = Spec { quote: |q| q.magic = 0xFF54_4348, ..Spec::default() };
    assert_eq!(refused(s), Refusal::Quote(QuoteFault::NotTpmGenerated));
}

#[test]
fn an_attestation_that_is_not_a_quote_is_refused() {
    // TPM_ST_ATTEST_CERTIFY, signed by the same key.
    let s = Spec { quote: |q| q.st = 0x8017, ..Spec::default() };
    assert_eq!(refused(s), Refusal::Quote(QuoteFault::NotAQuote));
}

#[test]
fn a_signed_structure_with_a_trailing_byte_is_refused() {
    assert_eq!(refused(Spec { quote: |q| q.trailing = vec![0], ..Spec::default() }), MALFORMED);
}

#[test]
fn a_structure_that_does_not_parse_is_refused() {
    assert_eq!(refused(Spec { quote: |q| q.count = Some(0), ..Spec::default() }), MALFORMED);
    assert_eq!(refused(Spec { quote: |q| q.count = Some(17), ..Spec::default() }), MALFORMED);
    let big = |q: &mut crate::build::attest::Quote| q.selections[0].1 = vec![0; 5];
    assert_eq!(refused(Spec { quote: big, ..Spec::default() }), MALFORMED);
    assert_eq!(refused(Spec { quote: |q| q.safe = 2, ..Spec::default() }), MALFORMED);
    let long = |q: &mut crate::build::attest::Quote| q.signer = vec![0; 67];
    assert_eq!(refused(Spec { quote: long, ..Spec::default() }), MALFORMED);
    let cut = Spec::default().build().doc;
    let d = nsh_attest::Doc::parse(&cut).expect("parses");
    for n in 0..d.attest.len() {
        let r = nsh_attest::quote::Attest::parse(&d.attest[..n]);
        assert!(r.is_err(), "an attest cut at {n} is refused");
    }
}
