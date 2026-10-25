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

use core::sync::atomic::{AtomicU64, Ordering};

static NONCE_COUNTER: AtomicU64 = AtomicU64::new(1);

/// A nonce that only has to be unique: random when the RNG answers, and
/// otherwise the next value of a counter, which is unique by itself.
#[inline]
pub fn default_nonce() -> u64 {
    let mut buf = [0u8; 8];
    match crate::crypto::random_api::get_bytes_secure(&mut buf) {
        Ok(()) => u64::from_le_bytes(buf),
        Err(_) => NONCE_COUNTER.fetch_add(1, Ordering::SeqCst),
    }
}

/// A nonce that must not be guessed: from the RNG, or an error. A failed
/// draw is never handed out as sixteen zero bytes.
#[inline]
pub(crate) fn secure_nonce_128() -> Result<[u8; 16], &'static str> {
    let mut buf = [0u8; 16];
    crate::crypto::random_api::get_bytes_secure(&mut buf).map_err(|_| "no randomness")?;
    Ok(buf)
}

#[inline]
pub fn current_nonce_counter() -> u64 {
    NONCE_COUNTER.load(Ordering::SeqCst)
}
