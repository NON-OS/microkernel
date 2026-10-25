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

//! A version 3 quote, over PCRs 0, 1, 2 and 7, still proves the signature
//! and digest code through `check_quote`, though no version 3 document is
//! ever accepted.

use nonos_hash::sha256;
use nsh_attest::consts::QUOTED_PCRS_V4;
use nsh_attest::quote::check_quote;
use nsh_attest::Refusal;

use crate::build::ak::Key;
use crate::build::attest::Quote;
use crate::build::sig::tpmt;

const V3: [u8; 4] = [0, 1, 2, 7];

fn v3_quote(values: &[[u8; 32]]) -> (Vec<u8>, Vec<u8>) {
    let attest = Quote::honest(&[0x42; 32], &V3, values).bytes();
    let (r, s) = Key::new(1).sign(&sha256(&attest), 7);
    (attest, tpmt(0x18, 0x0B, &r, &s))
}

type Setup = ([[u8; 32]; 4], Vec<u8>, Vec<u8>, [u8; 64]);

pub(super) fn setup() -> Setup {
    let values = [[1; 32], [2; 32], [3; 32], [4; 32]];
    let (attest, sig) = v3_quote(&values);
    (values, attest, sig, Key::new(1).public())
}

#[test]
fn a_version_3_quote_checks_over_its_own_pcrs() {
    let (values, attest, sig, ak) = setup();
    let refs: Vec<&[u8; 32]> = values.iter().collect();
    let a = check_quote(&attest, &sig, &ak, &[0x42; 32], &V3, &refs).expect("checks");
    assert_eq!(a.selection.bitmap, [0x87, 0, 0]);
    let mut wrong = values;
    wrong[3] = [5; 32];
    let wrong: Vec<&[u8; 32]> = wrong.iter().collect();
    let r = check_quote(&attest, &sig, &ak, &[0x42; 32], &V3, &wrong);
    assert_eq!(r.map(|_| ()), Err(Refusal::PcrDigest));
}

#[test]
fn a_version_3_quote_is_not_a_version_4_one() {
    let (values, attest, sig, ak) = setup();
    let refs: Vec<&[u8; 32]> = values.iter().collect();
    let r = check_quote(&attest, &sig, &ak, &[0x42; 32], &QUOTED_PCRS_V4, &refs);
    assert_eq!(r.map(|_| ()), Err(Refusal::Selection));
}
