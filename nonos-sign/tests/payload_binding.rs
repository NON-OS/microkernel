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

//! The payload binding the reuse gate stands on: a manifest names the
//! BLAKE3 of exactly one binary, and the decoded hash must equal a
//! fresh measurement of those bytes, nothing shorter, longer, or
//! bit-flipped. This is the library-level truth behind
//! `verify-manifest --elf`.

use nonos_capsule_sign::verify::decode::decode_manifest;

use super::fixtures::{
    cert_id_of, cert_input, make_bundle, manifest_input, sign_cert, sign_manifest_full,
};

const PAYLOAD: &[u8] = b"FAKE_ELF_PAYLOAD";

fn decoded_payload_hash() -> [u8; 32] {
    let b = make_bundle();
    let cert_bytes = sign_cert(cert_input(&b), &b);
    let manifest_bytes = sign_manifest_full(manifest_input(&b, cert_id_of(&cert_bytes)), &b);
    decode_manifest(&manifest_bytes).unwrap().payload_hash
}

#[test]
fn enrolled_bytes_measure_to_the_manifest() {
    assert_eq!(decoded_payload_hash(), *blake3::hash(PAYLOAD).as_bytes());
}

#[test]
fn a_flipped_bit_is_a_different_binary() {
    let mut tampered = PAYLOAD.to_vec();
    tampered[0] ^= 1;
    assert_ne!(decoded_payload_hash(), *blake3::hash(&tampered).as_bytes());
}

#[test]
fn truncation_is_a_different_binary() {
    assert_ne!(decoded_payload_hash(), *blake3::hash(&PAYLOAD[..PAYLOAD.len() - 1]).as_bytes());
}

#[test]
fn a_suffix_is_a_different_binary() {
    let mut extended = PAYLOAD.to_vec();
    extended.push(0);
    assert_ne!(decoded_payload_hash(), *blake3::hash(&extended).as_bytes());
}
