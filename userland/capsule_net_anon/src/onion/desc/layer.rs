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


//! One encrypted layer of a descriptor (decrypt_desc_layer in the fork).
//!
//! A layer is SALT (16) | ENCRYPTED | MAC (32). SHAKE-256 over the secret
//! data, the subcredential, the revision counter, the salt and the layer's
//! constant gives a 32 byte AES-256 key, a 16 byte IV and a 32 byte MAC key.
//! The MAC is checked before anything is decrypted.

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::equal;
use crate::crypto::keccak::{sha3_256_parts, Shake256};
use nonos_aes::Ctr256Be;

pub const SUPERENCRYPTED: &[u8] = b"hsdir-superencrypted-data";
pub const ENCRYPTED: &[u8] = b"hsdir-encrypted-data";
const SALT: usize = 16;
const MAC: usize = 32;

/// The plaintext of `blob`, cut at its first NUL as the fork cuts its
/// padding. `None` when the blob is too short, the MAC does not match or
/// nothing is left.
pub fn open(blob: &[u8], secret: &[u8], subcredential: &[u8; 32], revision: u64, constant: &[u8]) -> Option<Vec<u8>> {
    if blob.len() <= SALT + MAC {
        return None;
    }
    let salt = &blob[..SALT];
    let body = &blob[SALT..blob.len() - MAC];
    let mac = &blob[blob.len() - MAC..];

    let mut kdf = Shake256::new();
    kdf.update(secret);
    kdf.update(subcredential);
    kdf.update(&revision.to_be_bytes());
    kdf.update(salt);
    kdf.update(constant);
    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    let mut mac_key = [0u8; 32];
    kdf.squeeze(&mut key);
    kdf.squeeze(&mut iv);
    kdf.squeeze(&mut mac_key);

    let want = sha3_256_parts(&[&(MAC as u64).to_be_bytes(), &mac_key, &(SALT as u64).to_be_bytes(), salt, body]);
    wipe(&mut mac_key);
    if !equal(&want, mac) {
        wipe(&mut key);
        return None;
    }
    let mut plain = body.to_vec();
    Ctr256Be::with_iv(&key, &iv).apply(&mut plain);
    wipe(&mut key);
    if let Some(end) = plain.iter().position(|b| *b == 0) {
        plain.truncate(end);
    }
    (!plain.is_empty()).then_some(plain)
}

fn wipe(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        /* SAFETY: eK@nonos.systems. Layer keys, wiped with a store the
         * optimiser may not remove. */
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
