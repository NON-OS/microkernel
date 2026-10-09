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


//! Signed name lists made the way the fork's DNS services make them
//! (anyone_hosts_parse.c, test_hs_common.c's signed cases), with
//! ed25519-dalek and sha2, not the capsule's own code.

use std::string::String;
use std::vec::Vec;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest as _, Sha256};
use sha3::Sha3_256;

/// A throwaway signer from a label.
pub fn signer(label: &str) -> SigningKey {
    SigningKey::from_bytes(&super::world::keys_bytes32(label))
}

/// The `.anyone` address of an Ed25519 public key (rend-spec-v3 6 with the
/// fork's suffix and checksum prefix).
pub fn address_of(public: &[u8; 32]) -> String {
    let mut c = Sha3_256::new();
    sha3::Digest::update(&mut c, b".anyone checksum");
    sha3::Digest::update(&mut c, public);
    sha3::Digest::update(&mut c, [3u8]);
    let digest = sha3::Digest::finalize(c);
    let mut raw = public.to_vec();
    raw.extend_from_slice(&digest[..2]);
    raw.push(3);
    let alphabet = b"abcdefghijklmnopqrstuvwxyz234567";
    let (mut acc, mut bits, mut out) = (0u64, 0u32, String::new());
    for byte in raw {
        acc = (acc << 8) | u64::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(alphabet[((acc >> bits) & 31) as usize] as char);
        }
    }
    out + ".anyone"
}

/// The standard header: version, status, the two times.
pub fn header(published: &str, valid_until: &str) -> String {
    std::format!("anyone-hosts-version 1\nanyone-hosts-status signed\npublished {published}\nvalid-until {valid_until}\n")
}

/// `body` (everything before the signature line) signed by `key`, named as
/// `signer_address`, with the signature in a PEM block of 64-column lines.
pub fn sign_as(body: &str, key: &SigningKey, signer_address: &str) -> Vec<u8> {
    let mut region = String::from(body);
    region += &std::format!("anyone-hosts-signature {signer_address}\n");
    let mut message = b"anyone-hosts-signature".to_vec();
    message.extend_from_slice(&Sha256::digest(region.as_bytes()));
    let signature = key.sign(&message).to_bytes();
    let encoded = STANDARD.encode(signature);
    region += "-----BEGIN SIGNATURE-----\n";
    for chunk in encoded.as_bytes().chunks(64) {
        region += core::str::from_utf8(chunk).unwrap();
        region += "\n";
    }
    region += "-----END SIGNATURE-----\n";
    region.into_bytes()
}

pub fn sign(body: &str, key: &SigningKey) -> Vec<u8> {
    let address = address_of(&key.verifying_key().to_bytes());
    sign_as(body, key, &address)
}
