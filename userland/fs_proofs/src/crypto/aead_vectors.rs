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

//! RFC 8439 AEAD known answer (2.8.2) and the Poly1305 edge cases of
//! appendix A.3 (vectors 5 to 11), which drive the carries and the final
//! reduction through their corner values. The A.3 tags were also
//! recomputed with Python integers when these tests were written.

use super::chacha20poly1305::{aead_decrypt, aead_encrypt, poly1305_mac};
use super::test_input::{arr, unhex};

const PT: &[u8] = b"Ladies and Gentlemen of the class of '99: If I could offer you \
    only one tip for the future, sunscreen would be it.";
const CT_AND_TAG: &str = "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6\
    3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36\
    92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc\
    3ff4def08e4b7a9de576d26586cec64b61161ae10b594f09e26a7e902ecbd0600691";

#[test]
fn rfc8439_2_8_2_aead() {
    let key = arr(&unhex("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f"));
    let nonce = arr(&unhex("070000004041424344454647"));
    let aad = unhex("50515253c0c1c2c3c4c5c6c7");
    assert_eq!(aead_encrypt(&key, &nonce, &aad, PT).unwrap(), unhex(CT_AND_TAG));
    assert_eq!(aead_decrypt(&key, &nonce, &aad, &unhex(CT_AND_TAG)).unwrap(), PT);
}

const R1: &str = "01000000000000000000000000000000";
const R2: &str = "02000000000000000000000000000000";
const R10: &str = "01000000000000000400000000000000";
const ZERO: &str = "00000000000000000000000000000000";
const FF: &str = "ffffffffffffffffffffffffffffffff";
const M10: &str = "e33594d7505e43b900000000000000003394d7505e4379cd0100000000000000\
    00000000000000000000000000000000";

#[test]
fn rfc8439_a3_poly1305_edges() {
    let cases: [(&str, &str, String, &str); 7] = [
        (R2, ZERO, FF.into(), "03000000000000000000000000000000"),
        (R2, FF, R2.into(), "03000000000000000000000000000000"),
        (
            R1,
            ZERO,
            format!("{FF}f0{}11{}", &FF[2..], &ZERO[2..]),
            "05000000000000000000000000000000",
        ),
        (R1, ZERO, format!("{FF}fb{}{}", "fe".repeat(15), "01".repeat(16)), ZERO),
        (R2, ZERO, format!("fd{}", &FF[2..]), "faffffffffffffffffffffffffffffff"),
        (R10, ZERO, format!("{M10}01{}", &ZERO[2..]), "14000000000000005500000000000000"),
        (R10, ZERO, M10.into(), "13000000000000000000000000000000"),
    ];
    for (n, (r, s, msg, tag)) in cases.iter().enumerate() {
        let key = arr(&unhex(&format!("{r}{s}")));
        let got = poly1305_mac(&unhex(msg), &key);
        assert_eq!(got.to_vec(), unhex(tag), "A.3 vector {}", n + 5);
    }
}
