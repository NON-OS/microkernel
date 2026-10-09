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

use super::chacha_core::{keystream, setup};
use crate::crypto::constant_time::{compiler_fence, volatile_write};

pub const CHACHA20_BLOCK_SIZE: usize = 64;

#[inline]
pub(crate) fn secure_zero_bytes(buf: &mut [u8]) {
    for b in buf {
        // SAFETY: We have exclusive mutable access to buf, and volatile write ensures
        // the compiler cannot optimize away this zeroing operation.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    compiler_fence();
}

/// Writes the key-stream block number `counter` for `key` and `nonce`.
pub fn chacha20_block(key: &[u8; 32], nonce: &[u8; 12], counter: u32, out: &mut [u8; 64]) {
    let mut state = setup(key, nonce);
    state[12] = counter;
    let words = keystream(&state);
    for (dst, w) in out.as_chunks_mut::<4>().0.iter_mut().zip(words) {
        *dst = w.to_le_bytes();
    }
}

/// XORs the key stream, from block `counter` on, into `data`. Whole
/// blocks are XORed a word at a time with the key-stream words, never
/// serialized to a byte buffer; only a short tail goes through a stack
/// block. That block and the state, which holds the key, are wiped
/// before return.
pub(crate) fn chacha20_xor(key: &[u8; 32], nonce: &[u8; 12], counter: u32, data: &mut [u8]) {
    let mut state = setup(key, nonce);
    state[12] = counter;
    let (blocks, tail) = data.as_chunks_mut::<64>();
    for block in blocks {
        let words = keystream(&state);
        for (dst, w) in block.as_chunks_mut::<4>().0.iter_mut().zip(words) {
            *dst = (u32::from_le_bytes(*dst) ^ w).to_le_bytes();
        }
        state[12] = state[12].wrapping_add(1);
    }
    if !tail.is_empty() {
        let mut stream = [0u8; 64];
        chacha20_block(key, nonce, state[12], &mut stream);
        for (d, k) in tail.iter_mut().zip(stream.iter()) {
            *d ^= k;
        }
        secure_zero_bytes(&mut stream);
    }
    volatile_write(&mut state, [0; 16]);
}
