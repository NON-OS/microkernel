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

//! Step 3, the signature's shape: r and s of 1 to 32 bytes each, in range,
//! and nothing after them. A TPM that drops a leading zero byte is read.

use nonos_hash::sha256;
use nsh_attest::quote::check_quote;
use nsh_attest::Refusal;

use super::refused;
use crate::build::ak::Key;
use crate::build::attest::Quote;
use crate::build::sig::tpmt;
use crate::build::Spec;

fn sig(f: fn([u8; 32], [u8; 32]) -> Vec<u8>) -> Refusal {
    refused(Spec { sig: f, ..Spec::default() })
}

#[test]
fn malformed_signatures_are_refused() {
    let s = Refusal::Signature;
    assert_eq!(sig(|r, s| tpmt(0x18, 0x0B, &[&[0u8][..], &r].concat(), &s[1..])), s);
    assert_eq!(sig(|_, s| tpmt(0x18, 0x0B, &[], &s)), s);
    assert_eq!(sig(|r, _| tpmt(0x18, 0x0B, &r, &[])), s);
    assert_eq!(sig(|r, s| [tpmt(0x18, 0x0B, &r[1..], &s), vec![0]].concat()), s);
    assert_eq!(sig(|r, s| tpmt(0x18, 0x0B, &r, &s)[..70].to_vec()), s);
}

#[test]
fn a_short_r_with_its_leading_zero_dropped_is_read() {
    let (key, extra, values) = (Key::new(1), [0x42; 32], vec![[0x0A; 32]; 4]);
    let refs: Vec<&[u8; 32]> = values.iter().collect();
    let attest = Quote::honest(&extra, &[0, 1, 2, 7], &values).bytes();
    let digest = sha256(&attest);
    let (salt, r, s) = (0..4096)
        .map(|salt| (salt, key.sign(&digest, salt)))
        .find_map(|(n, (r, s))| (r[0] == 0).then_some((n, r, s)))
        .expect("one r in 256 starts with zero");
    let ak = key.public();
    for sig in [tpmt(0x18, 0x0B, &r[1..], &s), tpmt(0x18, 0x0B, &r, &s)] {
        let ok = check_quote(&attest, &sig, &ak, &extra, &[0, 1, 2, 7], &refs);
        assert!(ok.is_ok(), "salt {salt}");
    }
    let trailing = [tpmt(0x18, 0x0B, &r[1..], &s), vec![0]].concat();
    let r = check_quote(&attest, &trailing, &ak, &extra, &[0, 1, 2, 7], &refs);
    assert_eq!(r.map(|_| ()), Err(Refusal::Signature), "a good signature with a byte after it");
}
