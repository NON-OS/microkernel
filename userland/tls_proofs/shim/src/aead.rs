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

//! The AEAD syscalls, ChaCha20-Poly1305 only, as the record layer used them.

use crate::protocol::{OP_CHACHA20_POLY1305_OPEN, OP_CHACHA20_POLY1305_SEAL};
use crate::serve::{deliver, input, kernel_call};

/*
 * The caller's frame is `aad_len u32 LE || aad || payload`; the pool wants
 * `key || nonce || aad_len || aad || payload`. The kernel does that
 * rearranging, so this does too.
 */
fn call(op: u16, key: *const u8, nonce: *const u8, frame: &[u8], out: *mut u8) -> i64 {
    let mut body = input(key, 32).to_vec();
    body.extend_from_slice(input(nonce, 12));
    body.extend_from_slice(frame);
    match kernel_call(op, &body) {
        Ok(bytes) => deliver(&bytes, out),
        Err(status) => status,
    }
}

pub fn crypto_decrypt_aad(
    algo: u64,
    key: *const u8,
    nonce: *const u8,
    frame: *const u8,
    frame_len: usize,
    plaintext: *mut u8,
) -> i64 {
    if algo != 0 {
        return -22;
    }
    call(OP_CHACHA20_POLY1305_OPEN, key, nonce, input(frame, frame_len), plaintext)
}

pub fn crypto_encrypt_aad(
    algo: u64,
    key: *const u8,
    nonce: *const u8,
    frame: *const u8,
    frame_len: usize,
    ciphertext: *mut u8,
) -> i64 {
    if algo != 0 {
        return -22;
    }
    call(OP_CHACHA20_POLY1305_SEAL, key, nonce, input(frame, frame_len), ciphertext)
}
