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

//! The two core API calls the capability engine makes that are not the
//! point here. The kernel's key generator stirs CPU words and timing into a
//! seeded draw and refuses when that draw does; this one is that draw
//! alone. The region hash reads raw memory; this one hashes nothing.

use crate::crypto::error::{CryptoError, CryptoResult};

pub fn generate_secure_key() -> CryptoResult<[u8; 32]> {
    let mut key = [0u8; 32];
    super::rng::fill_random_bytes_secure(&mut key).map_err(|_| CryptoError::InsufficientEntropy)?;
    Ok(key)
}

pub fn hash_memory_region(
    _start: usize,
    _size: usize,
    out: &mut [u8; 32],
) -> Result<(), &'static str> {
    out.fill(0x11);
    Ok(())
}
