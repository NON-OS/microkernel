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
//! Host proofs for the software AES.

//! What net.nym's acknowledgement source asks of `crate::crypto`: its own
//! AES, compiled in unchanged, and a random source that differs per call.

#[path = "../../../capsule_net_nym/src/crypto/aes/mod.rs"]
pub mod aes;

pub mod types {
    #[derive(Debug)]
    pub enum CryptoError {
        Random,
    }
}

pub mod random {
    use super::types::CryptoError;
    use core::sync::atomic::{AtomicU8, Ordering};

    static NEXT: AtomicU8 = AtomicU8::new(1);

    /// A different fill each call, so two acks of one fragment differ.
    pub fn fill_random(out: &mut [u8]) -> Result<(), CryptoError> {
        let seed = NEXT.fetch_add(1, Ordering::Relaxed);
        if out.is_empty() {
            return Err(CryptoError::Random);
        }
        for (i, b) in out.iter_mut().enumerate() {
            *b = seed.wrapping_mul(31).wrapping_add(i as u8);
        }
        Ok(())
    }
}
