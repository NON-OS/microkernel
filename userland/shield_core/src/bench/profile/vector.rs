//! The 37-limb `transfer-eth` vector of the STARK repository at `1b4b5a3`: request, seed and
//! entropy byte for byte, and the hash of its `proof.json`. The seed opens two fixture notes under
//! a root the production pool does not know, so its secrets spend nothing.

use nox_prover::{Proof, ENTROPY_BYTES};
use sha3::{Digest, Keccak256};

pub const REQUEST: &str = include_str!("vector/request.json");
pub const SEED: &str = include_str!("vector/seed.json");
pub(super) const ENTROPY: &str = include_str!("vector/entropy.hex");

/// Keccak-256 and length of `proof.json` in the same vector.
const PROOF_JSON_KECCAK: [u8; 32] = [
    0x14, 0x06, 0x2a, 0xe3, 0xfd, 0x1a, 0x99, 0x32, 0x6a, 0x6f, 0x30, 0x5f, 0x78, 0x44, 0x40, 0xcf,
    0x17, 0x79, 0x2d, 0xa6, 0x7f, 0xd1, 0x88, 0xe0, 0x25, 0xa6, 0x7a, 0xb6, 0x42, 0xd9, 0xa6, 0x84,
];
const PROOF_JSON_BYTES: usize = 236_921;

pub fn entropy() -> Option<[u8; ENTROPY_BYTES]> {
    let mut out = [0u8; ENTROPY_BYTES];
    let digits = ENTROPY.trim().as_bytes();
    if digits.len() != ENTROPY_BYTES.checked_mul(2)? {
        return None;
    }
    for (byte, pair) in out.iter_mut().zip(digits.chunks_exact(2)) {
        *byte = u8::from_str_radix(core::str::from_utf8(pair).ok()?, 16).ok()?;
    }
    Some(out)
}

/// Whether `proof`, written as the vector writes it with its note secrets, is `proof.json`.
pub fn matches(proof: &Proof) -> bool {
    let json = zeroize::Zeroizing::new(nox_prover::to_json(proof));
    json.len() == PROOF_JSON_BYTES
        && Keccak256::digest(json.as_bytes()).as_slice() == PROOF_JSON_KECCAK
}
