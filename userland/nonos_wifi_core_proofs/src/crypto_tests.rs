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

//! The primitives the SHA-256 AKMs add, against published known answers.
//! AES-128-CMAC here: RFC 4493 section 4, examples 1 to 4. HMAC-SHA256,
//! HKDF and the PSK are in `crypto_kdf_tests`.

use crate::hex::h;
use nonos_wifi_core::ccmp::cmac::aes_cmac;

const CMAC_KEY: &str = "2b7e151628aed2a6abf7158809cf4f3c";
const CMAC_MSG: &str = "6bc1bee22e409f96e93d7e117393172a ae2d8a571e03ac9c9eb76fac45af8e51
    30c81c46a35ce411e5fbc1191a0a52ef f69f2445df4f9b17ad2b417be66c3710";

#[test]
fn aes_cmac_matches_rfc4493() {
    let key: [u8; 16] = h(CMAC_KEY).try_into().unwrap();
    let m = h(CMAC_MSG);
    let cases: [(usize, &str); 4] = [
        (0, "bb1d6929e95937287fa37d129b756746"),
        (16, "070a16b46b4d4144f79bdd9dd04a287c"),
        (40, "dfa66747de9ae63030ca32611497c827"),
        (64, "51f0bebf7e3b9d92fc49741779363cfe"),
    ];
    for (len, tag) in cases {
        assert_eq!(aes_cmac(&key, &m[..len]).to_vec(), h(tag), "message length {len}");
    }
}
