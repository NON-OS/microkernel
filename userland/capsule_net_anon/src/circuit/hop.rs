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

//! One hop's keystreams, digests and windows.

use nonos_aes::{Ctr128Be, Ctr256Be};

use crate::crypto::keccak::Sha3_256;
use crate::crypto::sha1::Sha1;
use crate::ntor::KEY_MATERIAL_BYTES;

use super::hop_crypto::{Keystream, RunningDigest};

/// One hop's crypto: a keystream and a running digest in each direction.
pub struct Hop {
    pub forward: Keystream,
    pub backward: Keystream,
    pub forward_digest: RunningDigest,
    pub backward_digest: RunningDigest,
    /// The digest of the last cell that arrived and was ours, kept because a
    /// SENDME has to name it.
    pub last_seen: [u8; 20],
    /// Cells we may still send before the far end has to grant more.
    pub package_window: i32,
    /// Cells that may still arrive before we owe a SENDME.
    pub deliver_window: i32,
}

impl Hop {
    /*
     * The key material is `Df || Db || Kf || Kb`: twenty bytes that seed the
     * forward digest, twenty that seed the backward one, then a sixteen byte
     * key each way. The digests are seeded by being fed their twenty bytes,
     * not by having their state overwritten, which is why this is an
     * `update` and not an assignment.
     */
    pub fn new(keys: &[u8; KEY_MATERIAL_BYTES]) -> Self {
        let mut forward_digest = Sha1::new();
        forward_digest.update(&keys[..20]);
        let mut backward_digest = Sha1::new();
        backward_digest.update(&keys[20..40]);
        let mut forward_key = [0u8; 16];
        forward_key.copy_from_slice(&keys[40..56]);
        let mut backward_key = [0u8; 16];
        backward_key.copy_from_slice(&keys[56..72]);
        let hop = Self::with(
            Keystream::Aes128(Ctr128Be::new(&forward_key)),
            Keystream::Aes128(Ctr128Be::new(&backward_key)),
            RunningDigest::Sha1(forward_digest),
            RunningDigest::Sha1(backward_digest),
        );
        /* The keys are in the ciphers' schedules now; the stack copies go. */
        crate::crypto::wipe::wipe(&mut forward_key);
        crate::crypto::wipe::wipe(&mut backward_key);
        hop
    }

    /*
     * The onion service's own hop, from the 128 bytes hs-ntor expands to:
     * `Df || Db` as 32 byte seeds of SHA3-256 digests, then `Kf || Kb` as
     * AES-256 keys. The client side keeps them in that order; only the
     * service reverses them (relay_crypto_init, reverse = is_service_side).
     */
    pub fn onion(keys: &[u8; 128]) -> Self {
        let mut forward_digest = Sha3_256::new();
        forward_digest.update(&keys[..32]);
        let mut backward_digest = Sha3_256::new();
        backward_digest.update(&keys[32..64]);
        let mut forward_key = [0u8; 32];
        forward_key.copy_from_slice(&keys[64..96]);
        let mut backward_key = [0u8; 32];
        backward_key.copy_from_slice(&keys[96..128]);
        let hop = Self::with(
            Keystream::Aes256(Ctr256Be::new(&forward_key)),
            Keystream::Aes256(Ctr256Be::new(&backward_key)),
            RunningDigest::Sha3(forward_digest),
            RunningDigest::Sha3(backward_digest),
        );
        for byte in forward_key.iter_mut().chain(backward_key.iter_mut()) {
            /* SAFETY: eK@nonos.systems. Copies of hop keys, now in the
             * cipher's schedule; the stack copies are wiped. */
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        hop
    }

    fn with(forward: Keystream, backward: Keystream, forward_digest: RunningDigest, backward_digest: RunningDigest) -> Self {
        Self {
            forward,
            backward,
            forward_digest,
            backward_digest,
            last_seen: [0u8; 20],
            package_window: super::window::CIRCUIT_START,
            deliver_window: super::window::CIRCUIT_START,
        }
    }
}
