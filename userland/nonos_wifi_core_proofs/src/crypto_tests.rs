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
//! AES-128-CMAC: RFC 4493 section 4, examples 1 to 4. HMAC-SHA256 and HKDF:
//! RFC 5869 appendix A.1. The 802.11 KDF over HMAC-SHA256 is pinned through
//! the SAE KCK/PMK vector in `sae_tests` (it is the KDF that derives them).
//! The PSK: 64 hexadecimal digits are the key, a passphrase goes through
//! PBKDF2 (the IEEE Std 802.11-2020 J.4.2 vector), anything else is refused.

use nonos_wifi_core::ccmp::cmac::aes_cmac;
use nonos_wifi_core::wpa::hkdf::{expand, extract};
use nonos_wifi_core::wpa::psk::psk;
use nonos_wifi_core::wpa::sha256::hmac_sha256;

fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

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

#[test]
fn hkdf_sha256_matches_rfc5869_case1() {
    let ikm = [0x0bu8; 22];
    let salt = h("000102030405060708090a0b0c");
    let info = h("f0f1f2f3f4f5f6f7f8f9");
    let prk = extract(&salt, &[&ikm]);
    assert_eq!(prk.to_vec(), h("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5"));
    let mut okm = [0u8; 42];
    assert!(expand(&prk, &info, &mut okm));
    assert_eq!(
        okm.to_vec(),
        h("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865")
    );
    // HMAC-SHA256 is the extract step with the salt as key.
    assert_eq!(hmac_sha256(&salt, &ikm), prk);
}

#[test]
fn a_psk_is_hex_or_a_passphrase_and_nothing_else() {
    // IEEE Std 802.11-2020 J.4.2: "password" on SSID "IEEE".
    let from_pass = psk(b"password", b"IEEE").expect("a valid passphrase");
    assert_eq!(
        from_pass.to_vec(),
        h("f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e")
    );
    // The same key typed as 64 hex digits is used as is, not hashed again.
    let hex = b"f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e";
    assert_eq!(psk(hex, b"another ssid"), Some(from_pass));
    assert!(psk(b"short", b"IEEE").is_none(), "under 8 characters");
    assert!(psk(&[b'z'; 64], b"IEEE").is_none(), "64 characters that are not hex");
    assert!(psk(b"tab\tinside!", b"IEEE").is_none(), "a control character");
}
