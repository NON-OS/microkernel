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

//! Drawing an Ed25519 key pair. The 32-byte seed comes from the seeded RNG
//! and the draw fails if the RNG cannot answer; nothing is made from weaker
//! bytes. The seed is wiped once the key pair holds it.

use super::KeyPair;
use crate::crypto::error::CryptoResult;

impl KeyPair {
    pub fn generate() -> CryptoResult<Self> {
        let mut seed = [0u8; 32];
        crate::crypto::random_api::get_bytes_secure(&mut seed)?;
        let pair = Self::from_seed(seed);
        for b in seed.iter_mut() {
            // SAFETY: a volatile write to a byte of a local array.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        Ok(pair)
    }
}
