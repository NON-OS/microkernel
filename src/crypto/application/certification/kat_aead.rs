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

//! ChaCha20-Poly1305 against the RFC 8439 answer, both directions.

use super::kat_hash::settle;
use super::rfc8439::{AAD, CIPHERTEXT_AND_TAG, KEY, NONCE, PLAINTEXT};
use super::{AlgorithmStatus, CRYPTO_STATE};

/// Encrypting the vector's plaintext must produce the vector's ciphertext and
/// tag, and decrypting the vector's ciphertext must give the plaintext back.
///
/// Only the first of those is a known-answer test. The previous form did
/// neither: it encrypted, decrypted its own output and compared that with what
/// it had started from, which holds for any implementation that is merely
/// consistent with itself. It also passed an empty associated-data string,
/// which is not this vector.
pub fn kat_chacha20poly1305() -> AlgorithmStatus {
    let sealed = match crate::crypto::chacha20poly1305::aead_encrypt(&KEY, &NONCE, &AAD, PLAINTEXT)
    {
        Ok(out) => out,
        Err(_) => return settle(&CRYPTO_STATE.chacha20poly1305, false),
    };
    if sealed.as_slice() != CIPHERTEXT_AND_TAG.as_slice() {
        return settle(&CRYPTO_STATE.chacha20poly1305, false);
    }
    let opened =
        crate::crypto::chacha20poly1305::aead_decrypt(&KEY, &NONCE, &AAD, &CIPHERTEXT_AND_TAG);
    let round_trips = matches!(opened, Ok(plain) if plain.as_slice() == PLAINTEXT);
    settle(&CRYPTO_STATE.chacha20poly1305, round_trips)
}
