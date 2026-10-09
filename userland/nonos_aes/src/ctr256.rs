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


//! Counter mode under AES-256, across the whole 128 bit block.

use crate::aes256::{Aes256, KEY256_BYTES};
use crate::types::BLOCK_BYTES;

/*
 * The same counter as Ctr128Be: the whole block, big endian, carried on
 * mid block from one call to the next. An onion service's virtual hop and
 * its INTRODUCE1 payload start it at zero; a descriptor layer starts it at
 * the IV its key derivation produced.
 */
pub struct Ctr256Be {
    cipher: Aes256,
    counter: [u8; BLOCK_BYTES],
    held: [u8; BLOCK_BYTES],
    used: usize,
}

impl Ctr256Be {
    /// A keystream under `key` with the counter at zero.
    pub fn new(key: &[u8; KEY256_BYTES]) -> Self {
        Self::with_iv(key, &[0u8; BLOCK_BYTES])
    }

    /// A keystream under `key` whose first counter block is `iv`.
    pub fn with_iv(key: &[u8; KEY256_BYTES], iv: &[u8; BLOCK_BYTES]) -> Self {
        Self { cipher: Aes256::new(key), counter: *iv, held: [0u8; BLOCK_BYTES], used: BLOCK_BYTES }
    }

    /// XOR the keystream into `data`, continuing where the last call stopped.
    pub fn apply(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            if self.used == BLOCK_BYTES {
                self.held = self.counter;
                self.cipher.encrypt_block(&mut self.held);
                self.used = 0;
                for step in self.counter.iter_mut().rev() {
                    *step = step.wrapping_add(1);
                    if *step != 0 {
                        break;
                    }
                }
            }
            *byte ^= self.held[self.used];
            self.used += 1;
        }
    }
}

impl Drop for Ctr256Be {
    fn drop(&mut self) {
        for byte in self.held.iter_mut() {
            /* SAFETY: eK@nonos.systems. Keystream, wiped like the key. */
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
