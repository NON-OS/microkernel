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

//! The P-256 verification against RFC 6979's published P-256 SHA-256
//! signatures, and its refusals.

use nonos_hash::sha256;
use nsh_attest::p256::verify;

fn hex<const N: usize>(s: &str) -> [u8; N] {
    let b: Vec<u8> =
        (0..N).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect();
    b.try_into().expect("length")
}

fn key() -> [u8; 64] {
    let x = "60FED4BA255A9D31C961EB74C6356D68C049B8923B61FA6CE669622E60F29FB6";
    let y = "7903FE1008B8BC99A41AE9E95628BC64F2F1B20C2D7E9F5177A3C294D4462299";
    hex(&format!("{x}{y}"))
}

const SAMPLE: (&str, &str) = (
    "EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716",
    "F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8",
);
const TEST: (&str, &str) = (
    "F1ABB023518351CD71D881567B1EA663ED3EFCF6C5132B354F28D3B0B7D38367",
    "019F4113742A2B14BD25926B49C649155F267E60D3814B4C0CC84250E46F0083",
);

#[test]
fn rfc6979_signatures_verify() {
    for (msg, (r, s)) in [(&b"sample"[..], SAMPLE), (b"test", TEST)] {
        assert!(verify(&key(), &sha256(msg), &hex(r), &hex(s)));
    }
}

#[test]
fn a_signature_over_another_message_or_changed_is_refused() {
    let (r, s) = (hex::<32>(SAMPLE.0), hex::<32>(SAMPLE.1));
    assert!(!verify(&key(), &sha256(b"test"), &r, &s));
    for i in 0..32 {
        let (mut r2, mut s2) = (r, s);
        r2[i] ^= 1;
        s2[i] ^= 0x80;
        assert!(!verify(&key(), &sha256(b"sample"), &r2, &s));
        assert!(!verify(&key(), &sha256(b"sample"), &r, &s2));
    }
}

#[test]
fn a_key_off_the_curve_or_out_of_the_field_is_refused() {
    let (r, s) = (hex::<32>(SAMPLE.0), hex::<32>(SAMPLE.1));
    let mut k = key();
    k[63] ^= 1;
    assert!(!verify(&k, &sha256(b"sample"), &r, &s));
    assert!(!verify(&[0xFF; 64], &sha256(b"sample"), &r, &s));
    assert!(!verify(&[0; 64], &sha256(b"sample"), &r, &s));
}
