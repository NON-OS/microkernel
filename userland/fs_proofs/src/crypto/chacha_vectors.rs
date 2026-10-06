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

//! RFC 8439 known answers for the kernel's ChaCha20 block (2.3.2),
//! ChaCha20 encryption (2.4.2) and Poly1305 (2.5.2).

use super::chacha20poly1305::{aead_encrypt, chacha20_block, poly1305_mac};
use super::test_input::{arr, unhex};

const SUNSCREEN: &[u8] = b"Ladies and Gentlemen of the class of '99: If I could offer you \
    only one tip for the future, sunscreen would be it.";

fn key_00_1f() -> [u8; 32] {
    core::array::from_fn(|i| i as u8)
}

#[test]
fn rfc8439_2_3_2_block() {
    let mut out = [0u8; 64];
    chacha20_block(&key_00_1f(), &arr(&unhex("000000090000004a00000000")), 1, &mut out);
    let want = unhex(
        "10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4e\
         d2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e",
    );
    assert_eq!(out.to_vec(), want);
}

const CT_2_4_2: &str = "6e2e359a2568f98041ba0728dd0d6981e97e7aec1d4360c20a27afccfd9fae0b\
    f91b65c5524733ab8f593dabcd62b3571639d624e65152ab8f530c359f0861d8\
    07ca0dbf500d6a6156a38e088a22b65e52bc514d16ccf806818ce91ab7793736\
    5af90bbf74a35be6b40b8eedf2785e42874d";

/*
 * 2.4.2 starts the key stream at block 1, as the AEAD does for its body,
 * so it is checked both block by block and through the AEAD's XOR path.
 */
#[test]
fn rfc8439_2_4_2_encryption() {
    let (key, nonce) = (key_00_1f(), arr(&unhex("000000000000004a00000000")));
    let mut by_block = SUNSCREEN.to_vec();
    for (i, chunk) in by_block.chunks_mut(64).enumerate() {
        let mut ks = [0u8; 64];
        chacha20_block(&key, &nonce, 1 + i as u32, &mut ks);
        chunk.iter_mut().zip(ks).for_each(|(b, k)| *b ^= k);
    }
    assert_eq!(by_block, unhex(CT_2_4_2));
    let sealed = aead_encrypt(&key, &nonce, &[], SUNSCREEN).unwrap();
    assert_eq!(sealed[..SUNSCREEN.len()], unhex(CT_2_4_2)[..]);
}

#[test]
fn rfc8439_2_5_2_poly1305() {
    let key = unhex("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
    let tag = poly1305_mac(b"Cryptographic Forum Research Group", &arr(&key));
    assert_eq!(tag.to_vec(), unhex("a8061dc1305136c6c22b8baf0c0127a9"));
}
