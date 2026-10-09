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

//! Signatures, against bytes written from the specification.

use super::parse_util::resp;
use crate::security::tpm::enroll::sign::parse_sign;

fn signed(alg: u16, r: &[u8], s: &[u8]) -> Vec<u8> {
    let mut b = vec![0, 0, 0, 0];
    b.extend_from_slice(&alg.to_be_bytes());
    b.extend_from_slice(&[0x00, 0x0B]);
    for half in [r, s] {
        b.extend_from_slice(&(half.len() as u16).to_be_bytes());
        b.extend_from_slice(half);
    }
    resp(0, &b)
}

#[test]
fn signatures_are_left_padded_and_strictly_shaped() {
    let sig = parse_sign(&signed(0x0018, &[1; 31], &[2; 32])).expect("a short r");
    assert_eq!((sig[0], &sig[1..32], &sig[32..]), (0, &[1u8; 31][..], &[2u8; 32][..]));
    for (alg, r) in [(0x0018, vec![1; 33]), (0x0018, vec![]), (0x0014, vec![1; 32])] {
        assert!(parse_sign(&signed(alg, &r, &[2; 32])).is_err(), "{alg:#x} r of {}", r.len());
    }
    let whole = signed(0x0018, &[1; 32], &[2; 32]);
    for cut in 0..whole.len() {
        assert!(parse_sign(&whole[..cut]).is_err(), "cut at {cut}");
    }
}
