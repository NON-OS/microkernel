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

//! The kernel's public keys as the tree holds them, and the two signature
//! checks the loader makes; a key id not the tree's is refused first.

use super::kernel::Attested;
use ed25519_dalek::{Signature, VerifyingKey};
use nonos_capsule_sign::algs::{mldsa65::MlDsa65, AlgId, Verifier};
use nonos_capsule_sign::keys::read_pub;
use std::path::Path;

pub const ED_PUB: &str = "nonos-data/trust/keys/kernel_signing_ed25519.pub";
pub const PQ_PUB: &str = "nonos-data/trust/keys/kernel_mldsa65.pub";

pub struct KernelKeys {
    ed: Vec<u8>,
    pq: Vec<u8>,
}

impl KernelKeys {
    pub fn load(tree: &Path) -> Result<Self, String> {
        Ok(Self { ed: key(tree, ED_PUB, AlgId::Ed25519)?, pq: key(tree, PQ_PUB, AlgId::MlDsa65)? })
    }
    pub fn verify(&self, a: &Attested<'_>) -> Result<(), String> {
        if key_id("NONOS:KEYID:ED25519:v1", &self.ed) != a.ed_key_id {
            return Err(
                "its Ed25519 signature is by a key that is not the tree's kernel key".into()
            );
        }
        if key_id("NONOS:KEYID:MLDSA65:v1", &self.pq) != a.pq_key_id {
            return Err(
                "its ML-DSA-65 signature is by a key that is not the tree's kernel key".into()
            );
        }
        let mut msg = blake3::hash(a.kernel).as_bytes().to_vec();
        msg.extend_from_slice(&a.rollback.to_le_bytes());
        let vk = <[u8; 32]>::try_from(self.ed.as_slice()).ok();
        let vk = vk.and_then(|k| VerifyingKey::from_bytes(&k).ok());
        let sig = <[u8; 64]>::try_from(a.ed_sig).ok().map(|s| Signature::from_bytes(&s));
        match (vk, sig) {
            (Some(vk), Some(sig)) if vk.verify_strict(&msg, &sig).is_ok() => {}
            _ => return Err("its Ed25519 signature does not verify".into()),
        }
        match MlDsa65::verify(&self.pq, &msg, a.pq_sig) {
            Ok(true) => Ok(()),
            _ => Err("its ML-DSA-65 signature does not verify".into()),
        }
    }
}

fn key(tree: &Path, rel: &str, alg: AlgId) -> Result<Vec<u8>, String> {
    let k = read_pub(&tree.join(rel)).map_err(|e| format!("{rel}: {e}"))?;
    match k.alg == alg {
        true => Ok(k.bytes.clone()),
        false => Err(format!("{rel} is not a {} key", alg.label())),
    }
}

pub fn key_id(context: &str, public: &[u8]) -> [u8; 32] {
    *blake3::Hasher::new_derive_key(context).update(public).finalize().as_bytes()
}
