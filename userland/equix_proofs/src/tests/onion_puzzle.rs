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


//! The onion-service puzzle at the Equi-X level: the solutions the fork's
//! solve loop found, and the solutions in the fork's test_hs_pow.c vectors,
//! verify under the 100-byte challenge "Tor hs intro v1\0" | blinded key |
//! seed | nonce | effort, and pass the effort check
//! BLAKE2b-32(challenge | solution) * effort <= 2^32 - 1.
//!
//! The capsule's own puzzle code is proved against the same lines in
//! anon_ntor_proofs; this file proves the crate under it.

use super::vectors::pows;
use crate::hex::hex;
use nonos_equix::{verify, Blake2b, Solution};

fn challenge(blinded: &[u8], seed: &[u8], nonce: &[u8], effort: u32) -> Vec<u8> {
    let mut c = b"Tor hs intro v1\0".to_vec();
    c.extend_from_slice(blinded);
    c.extend_from_slice(seed);
    c.extend_from_slice(nonce);
    c.extend_from_slice(&effort.to_be_bytes());
    assert_eq!(c.len(), 100);
    c
}

fn meets(challenge: &[u8], solution: &[u8; 16], effort: u32) -> bool {
    let mut h = Blake2b::new(4, &[0; 16]);
    h.update(challenge);
    h.update(solution);
    let mut r = [0u8; 4];
    h.finish(&mut r);
    u64::from(u32::from_be_bytes(r)) * u64::from(effort) <= u64::from(u32::MAX)
}

#[test]
fn every_solve_loop_answer_verifies_and_meets_its_effort() {
    let all = pows();
    assert_eq!(all.len(), 5);
    for p in all {
        let c = challenge(&p.blinded, &p.seed, &p.nonce, p.effort);
        assert_eq!(verify(&c, &Solution::from_bytes(&p.solution)), Ok(()));
        assert!(meets(&c, &p.solution, p.effort), "effort {}", p.effort);
        // hs_pow.c steps the nonce as a little-endian counter, once per
        // solve that found nothing good enough.
        let mut n = p.first_nonce;
        for _ in 1..p.solves {
            for byte in n.iter_mut() {
                *byte = byte.wrapping_add(1);
                if *byte != 0 {
                    break;
                }
            }
        }
        assert_eq!(n, p.nonce);
    }
}

/// test_hs_pow.c's accepted vectors: claimed effort, seed, blinded id, nonce
/// and solution.
const TOR_ACCEPTED: [(u32, &str, &str, &str, &str); 3] = [
    (
        0,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "55555555555555555555555555555555",
        "4312f87ceab844c78e1c793a913812d7",
    ),
    (
        1000000,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "59217255555555555555555555555555",
        "0f3db97b9cac20c1771680a1a34848d3",
    ),
    (
        100000,
        "86fb0acf4932cda44dbb451282f415479462dd10cb97ff5e7e8e2a53c3767a7f",
        "bfd298428562e530c52bdb36d81a0e293ef4a0e94d787f0f8c0c611f4f9e78ed",
        "2eff9fdbc34326d9d2f18ed277469c63",
        "400cb091139f86b352119f6e131802d6",
    ),
];

#[test]
fn tor_accepted_vectors_verify_and_meet_their_effort() {
    for (effort, seed, id, nonce, sol) in TOR_ACCEPTED {
        let c = challenge(&hex(id), &hex(seed), &hex(nonce), effort);
        let sol: [u8; 16] = hex(sol).try_into().unwrap();
        assert_eq!(verify(&c, &Solution::from_bytes(&sol)), Ok(()), "effort {effort}");
        assert!(meets(&c, &sol, effort), "effort {effort}");
    }
}

#[test]
fn tor_rejected_vectors_fail_where_tor_says() {
    // Claimed 99999 with a solution made for 100000: the effort is part of
    // the challenge, so the Equi-X solution itself no longer verifies.
    let (_, seed, id, nonce, sol) = TOR_ACCEPTED[2];
    let sol: [u8; 16] = hex(sol).try_into().unwrap();
    let c = challenge(&hex(id), &hex(seed), &hex(nonce), 99999);
    assert!(verify(&c, &Solution::from_bytes(&sol)).is_err());
    // The same solution under a nonce with one nibble changed.
    let c = challenge(&hex(id), &hex(seed), &hex("2eff9fdbc34326d9a2f18ed277469c63"), 100000);
    assert!(verify(&c, &Solution::from_bytes(&sol)).is_err());
    // All zero, claimed effort 1.
    let c = challenge(&hex(&"11".repeat(32)), &[0; 32], &[0; 16], 1);
    assert!(verify(&c, &Solution::default()).is_err());
}
