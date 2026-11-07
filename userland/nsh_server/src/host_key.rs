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

//! Host keys held in memory, for a host whose keys are not in a TPM.

use crate::{HostSig, HostSigner, Rand};
use alloc::boxed::Box;
use nonos_ed25519::KeyPair;
use nonos_mldsa65::SecretKey;
use nsh_crypto::Secret;
use nsh_wire::keys::{ED_PUBLIC_LEN, ML_PUBLIC_LEN};

/// Host keys held in memory: Ed25519 from a seed, ML-DSA-65 as given.
pub struct HostKey {
    ed: KeyPair,
    ml: Box<SecretKey>,
    ml_public: Box<[u8; ML_PUBLIC_LEN]>,
}

impl HostKey {
    pub fn from_parts(ed_seed: Secret<32>, ml: SecretKey, ml_public: &[u8; ML_PUBLIC_LEN]) -> Self {
        let mut seed = *ed_seed.expose();
        let ed = KeyPair::from_seed(seed);
        nonos_hash::wipe(&mut seed);
        HostKey { ed, ml: Box::new(ml), ml_public: Box::new(*ml_public) }
    }
}

impl HostSigner for HostKey {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN] {
        &self.ed.public
    }
    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN] {
        &self.ml_public
    }
    fn sign(&self, digest: &[u8; 64], rand: Rand<'_>) -> Option<HostSig> {
        let ml = nonos_mldsa65::sign(&self.ml, digest, rand).ok()?;
        let ed = nonos_ed25519::sign(&self.ed, digest).to_bytes();
        Some(HostSig { ed, ml: Box::new(ml) })
    }
}
