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

//! The three primitives the chain check needs, from the kernel's own code.

extern crate alloc;

#[path = "../../../capsule_net_anon/src/crypto/compare.rs"]
mod compare;

pub use compare::equal;

/// Why a primitive refused. Only `Digest` is reachable from the chain check.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CryptoError {
    Digest,
}

/// The kernel's SHA-256, so the digest a link certificate is compared against is
/// computed by real code.
pub fn sha256(data: &[u8]) -> Result<[u8; 32], CryptoError> {
    Ok(crypto_proofs::crypto::hash::sha256(data))
}

/*
 * The kernel's Ed25519 verify blocks for ever unless something has signed first.
 * ge_scalarmult_base_ct waits on the PRECOMP Once, and ensure_precomp is called
 * only from sign, so a process that only verifies spins in that wait. One
 * throwaway signature initialises it.
 *
 * The capsule does not have this problem at runtime: its verify goes to the
 * crypto pool, which uses ed25519-dalek. The kernel path is used here because
 * it is real NONOS code, and the priming is a workaround for that bug, not part
 * of the code under test.
 */
fn primed() {
    use crypto_proofs::crypto::asymmetric::ed25519::{sign, KeyPair};
    use spin::Once;
    static ONCE: Once<()> = Once::new();
    ONCE.call_once(|| {
        let _ = sign(&KeyPair::from_seed([7u8; 32]), b"prime the precomputed table");
    });
}

/// The kernel's Ed25519 verify, reached through its own signature type.
pub fn ed25519_verify(public: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    use crypto_proofs::crypto::asymmetric::ed25519::{verify, Signature};
    primed();
    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(signature);
    verify(public, message, &Signature::from_bytes(&bytes))
}
