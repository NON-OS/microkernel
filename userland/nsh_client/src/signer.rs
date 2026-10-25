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

//! The user's key, which signs `USER_AUTH`; `UserKey` wipes both halves.

use alloc::boxed::Box;
use nonos_ed25519::KeyPair;
use nonos_mldsa65::SecretKey;
use nsh_crypto::Secret;
use nsh_wire::keys::{ED_PUBLIC_LEN, ED_SIG_LEN, ML_PUBLIC_LEN, ML_SIG_LEN};

/// A source of fresh random bytes: it fills the slice, or says it cannot.
pub type Rand<'a> = &'a mut dyn FnMut(&mut [u8]) -> bool;

pub struct UserSig {
    pub ed: [u8; ED_SIG_LEN],
    pub ml: Box<[u8; ML_SIG_LEN]>,
}

pub trait UserSigner {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN];
    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN];
    /// Both halves over `digest`, or `None` if either could not be made.
    fn sign(&self, digest: &[u8; 64], rand: Rand<'_>) -> Option<UserSig>;
}

pub struct UserKey {
    ed: KeyPair,
    ml: Box<SecretKey>,
    ml_public: Box<[u8; ML_PUBLIC_LEN]>,
}

impl UserKey {
    /// A new key pair: Ed25519 from `ed_seed`, ML-DSA-65 from `rand`.
    pub fn generate(ed_seed: Secret<32>, rand: Rand<'_>) -> Option<Self> {
        let (ml_public, ml) = nonos_mldsa65::keypair(rand).ok()?;
        Some(UserKey::from_parts(ed_seed, ml, &ml_public))
    }

    pub fn from_parts(ed_seed: Secret<32>, ml: SecretKey, ml_public: &[u8; ML_PUBLIC_LEN]) -> Self {
        let mut seed = *ed_seed.expose();
        let ed = KeyPair::from_seed(seed);
        nonos_hash::wipe(&mut seed);
        UserKey { ed, ml: Box::new(ml), ml_public: Box::new(*ml_public) }
    }
}

impl UserSigner for UserKey {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN] {
        &self.ed.public
    }

    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN] {
        &self.ml_public
    }

    fn sign(&self, digest: &[u8; 64], rand: Rand<'_>) -> Option<UserSig> {
        let ml = nonos_mldsa65::sign(&self.ml, digest, rand).ok()?;
        let ed = nonos_ed25519::sign(&self.ed, digest).to_bytes();
        Some(UserSig { ed, ml: Box::new(ml) })
    }
}
