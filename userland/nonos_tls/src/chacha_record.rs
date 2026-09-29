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

//! ChaCha20-Poly1305 for one record, computed in the caller.

use alloc::vec::Vec;

use chacha20poly1305::aead::{AeadInPlace, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, Tag};

/*
 * Every record went to the crypto pool as its own syscall, kernel round trip
 * included, and the record key had to be copied out to it each time. The key
 * already lives in this process, so the cipher runs where the key is.
 */
/// Open `body` (ciphertext then 16 byte tag) with `aad` as the record header.
pub fn open(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], body: &[u8]) -> Option<Vec<u8>> {
    let split = body.len().checked_sub(16)?;
    let mut plain = body[..split].to_vec();
    let tag = Tag::from_slice(&body[split..]);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    cipher.decrypt_in_place_detached(Nonce::from_slice(nonce), aad, &mut plain, tag).ok()?;
    Some(plain)
}

/// Seal `plain`, returning ciphertext then tag.
pub fn seal(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], plain: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(plain.len() + 16);
    out.extend_from_slice(plain);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let tag = cipher.encrypt_in_place_detached(Nonce::from_slice(nonce), aad, &mut out).ok()?;
    out.extend_from_slice(&tag);
    Some(out)
}
