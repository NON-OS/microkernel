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

//! The three crypto calls ramfs seals its files with, shaped as the kernel's
//! are: the sealed form is the plaintext's length plus a sixteen-byte tag,
//! and opening refuses a form shorter than the tag or one whose tag does not
//! match. The cipher is no cipher; the ramfs proofs are about sizes and
//! refusals, and crypto_proofs holds the real AEAD to its vectors.

use std::cell::Cell;

const TAG: usize = 16;

thread_local! {
    static DRAWN: Cell<u8> = const { Cell::new(0) };
}

pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    /*
     * SAFETY: the caller hands `len` writable bytes at `buf`, as the
     * kernel call requires.
     */
    let out = unsafe { std::slice::from_raw_parts_mut(buf, len) };
    DRAWN.with(|d| {
        for b in out.iter_mut() {
            d.set(d.get().wrapping_add(1));
            *b = d.get();
        }
    });
    len as i64
}

fn tag(key: *const u8, nonce: *const u8, plain: &[u8]) -> [u8; TAG] {
    /*
     * SAFETY: ramfs passes its 32-byte key and 12-byte nonce.
     */
    let (key, nonce) =
        unsafe { (std::slice::from_raw_parts(key, 32), std::slice::from_raw_parts(nonce, 12)) };
    let mut t = [0u8; TAG];
    for (i, b) in key.iter().chain(nonce).chain(plain).enumerate() {
        t[i % TAG] = t[i % TAG].rotate_left(3) ^ b;
    }
    t
}

pub fn crypto_encrypt(
    _algo: u64,
    key: *const u8,
    nonce: *const u8,
    plaintext: *const u8,
    plaintext_len: u64,
    ciphertext: *mut u8,
) -> i64 {
    let len = plaintext_len as usize;
    /*
     * SAFETY: the caller hands `len` readable bytes at `plaintext` and
     * `len` plus the tag writable at `ciphertext`, as the kernel call
     * requires.
     */
    let (plain, out) = unsafe {
        (
            std::slice::from_raw_parts(plaintext, len),
            std::slice::from_raw_parts_mut(ciphertext, len + TAG),
        )
    };
    out[..len].copy_from_slice(plain);
    out[len..].copy_from_slice(&tag(key, nonce, plain));
    (len + TAG) as i64
}

pub fn crypto_decrypt(
    _algo: u64,
    key: *const u8,
    nonce: *const u8,
    ciphertext: *const u8,
    ciphertext_len: u64,
    plaintext: *mut u8,
) -> i64 {
    let len = ciphertext_len as usize;
    let Some(plain_len) = len.checked_sub(TAG) else { return -74 };
    /*
     * SAFETY: as the kernel call requires, `len` readable bytes at
     * `ciphertext` and room for the plaintext at `plaintext`.
     */
    let (sealed, out) = unsafe {
        (
            std::slice::from_raw_parts(ciphertext, len),
            std::slice::from_raw_parts_mut(plaintext, plain_len),
        )
    };
    if tag(key, nonce, &sealed[..plain_len]) != sealed[plain_len..] {
        return -74;
    }
    out.copy_from_slice(&sealed[..plain_len]);
    plain_len as i64
}
