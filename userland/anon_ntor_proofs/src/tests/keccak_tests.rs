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
//! Join.
//! SHA3-256, SHAKE-256 and Ed25519 key blinding, for onion services.
//!
//! The hash outputs are Python's hashlib over the same inputs. The blinding
//! answer is Tor's own (test_blinding_basics in the fork's
//! src/test/test_hs_common.c), computed with the keccak code under test.

use alloc::vec::Vec;

use crate::hex;
use crate::crypto::keccak::{mac_sha3_256, sha3_256_parts, Sha3_256, Shake256};

fn sha3_256(data: &[u8]) -> [u8; 32] {
    sha3_256_parts(&[data])
}

#[test]
fn sha3_256_against_hashlib() {
    let cases: [(Vec<u8>, &str); 6] = [
        (Vec::new(), "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"),
        (b"abc".to_vec(), "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"),
        ((0..=255u8).cycle().take(768).collect(), "c043b2b15d405c9f4cd92fdaef420eba6201d328fb34ec0e2c16e4981b9e4b39"),
        (alloc::vec![b'a'; 135], "8094bb53c44cfb1e67b7c30447f9a1c33696d2463ecc1d9c92538913392843c9"),
        (alloc::vec![b'a'; 136], "3fc5559f14db8e453a0a3091edbd2bc25e11528d81c66fa570a4efdcc2695ee1"),
        (alloc::vec![b'a'; 137], "f8d6846cedd2ccfadf15c5879ef95af724d799eed7391fb1c91f95344e738614"),
    ];
    for (input, want) in cases.iter() {
        assert_eq!(sha3_256(input).to_vec(), hex(want), "{} bytes", input.len());
    }
}

#[test]
fn split_input_and_peek_agree_with_one_pass() {
    let input: Vec<u8> = (0..=255u8).cycle().take(768).collect();
    let mut h = Sha3_256::new();
    for piece in input.chunks(7) {
        h.update(piece);
    }
    assert_eq!(h.peek(), sha3_256(&input));
    assert_eq!(h.finish(), sha3_256_parts(&[&input[..100], &input[100..]]));
}

#[test]
fn shake_256_against_hashlib() {
    let mut out = [0u8; 64];
    Shake256::new().squeeze(&mut out);
    assert_eq!(out.to_vec(), hex("46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be"));

    /* 300 bytes squeezed in uneven pieces, across two rate blocks. */
    let input: Vec<u8> = (0..=255u8).cycle().take(768).collect();
    let mut x = Shake256::new();
    x.update(&input);
    let mut long = alloc::vec![0u8; 300];
    let (a, b) = long.split_at_mut(137);
    x.squeeze(a);
    x.squeeze(b);
    assert_eq!(long[280..].to_vec(), hex("df9f4b19e0f2bb2035a07d8b5db20bcbade3ac92"));
}

#[test]
fn mac_is_length_key_then_message() {
    let key = [7u8; 32];
    let msg = b"tor-hs-ntor-curve25519-sha3-256-1:hs_mac";
    let mut whole = (key.len() as u64).to_be_bytes().to_vec();
    whole.extend_from_slice(&key);
    whole.extend_from_slice(msg);
    assert_eq!(mac_sha3_256(&key, msg), sha3_256(&whole));
}

const BASEPOINT: &[u8] = b"(15112221349535400772501151409588531511454012693041857206046113283949847762202, 46316835694926478169428394003475163141307993866256225615783033603165251855960)";

/// Tor's blinding vector: identity 833990B0..., time period 1234, length
/// 1440 minutes.
#[test]
fn blinding_matches_tor() {
    let public: [u8; 32] =
        hex("833990B085C1A688C1D4C8B1F6B56AFAF5A2ECA674449E1D704F83765CCB7BC6").try_into().unwrap();
    let mut nonce = b"key-blind".to_vec();
    nonce.extend_from_slice(&1234u64.to_be_bytes());
    nonce.extend_from_slice(&1440u64.to_be_bytes());
    let param = sha3_256_parts(&[b"Derive temporary signing key\0", &public, BASEPOINT, &nonce]);
    assert_eq!(param.to_vec(), hex("379E50DB31FEE6775ABD0AF6FB7C371E060308F4F847DB09FE4CFE13AF602287"));

    let blinded = nonos_ed25519::blind_public(&public, &param).expect("on the curve");
    assert_eq!(blinded.to_vec(), hex("3A50BF210E8F9EE955AE0014F7A6917FB65EBF098A86305ABB508D1A7291B6D5"));

    let credential = sha3_256_parts(&[b"credential", &public]);
    let subcredential = sha3_256_parts(&[b"subcredential", &credential, &blinded]);
    assert_eq!(subcredential.to_vec(), hex("635D55907816E8D76398A675A50B1C2F3E36B42A5CA77BA3A0441285161AE07D"));
}

#[test]
fn a_key_off_the_curve_is_refused() {
    let mut bad = [0u8; 32];
    bad[0] = 2;
    assert!(nonos_ed25519::blind_public(&bad, &[1u8; 32]).is_none());
}
