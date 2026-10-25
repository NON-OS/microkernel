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

//! Step 3, the digest: over the document's values, every one, in order.

use nsh_attest::quote::pcr_digest;
use nsh_attest::Refusal;

use super::refused;
use crate::build::attest::Quote;
use crate::build::Spec;

fn select(f: fn(&mut Quote)) -> Refusal {
    refused(Spec { quote: f, ..Spec::default() })
}

#[test]
fn values_the_quote_did_not_sign_are_refused() {
    for i in nsh_attest::consts::QUOTED_PCRS_V4 {
        let s = Spec { quoted: vec![(i, [0x5E; 32])], ..Spec::default() };
        assert_eq!(refused(s), Refusal::PcrDigest, "PCR {i}");
    }
}

#[test]
fn a_digest_that_leaves_a_pcr_out_is_refused() {
    let left_out = |q: &mut Quote| {
        let v: Vec<[u8; 32]> = Spec::default().quoted_values();
        let refs: Vec<&[u8; 32]> = v.iter().take(7).collect();
        q.digest = pcr_digest(&refs).to_vec();
    };
    assert_eq!(select(left_out), Refusal::PcrDigest);
    assert_eq!(select(|q| q.digest.truncate(20)), Refusal::PcrDigest);
}

#[test]
fn a_digest_over_the_values_out_of_order_is_refused() {
    let shuffled = |q: &mut Quote| {
        let mut v: Vec<[u8; 32]> = Spec::default().quoted_values();
        v.swap(5, 6);
        q.digest = pcr_digest(&v.iter().collect::<Vec<_>>()).to_vec();
    };
    assert_eq!(select(shuffled), Refusal::PcrDigest);
}
