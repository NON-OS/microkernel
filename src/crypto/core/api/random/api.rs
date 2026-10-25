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

//! The core API's random calls. Each draws from the seeded RNG only, and
//! refuses with `InsufficientEntropy` before it is seeded; none falls back.

use crate::crypto::error::{CryptoError, CryptoResult};
use crate::crypto::util::rng;

pub fn fill_random(buf: &mut [u8]) -> CryptoResult<()> {
    rng::fill_random_bytes_secure(buf).map_err(|_| CryptoError::InsufficientEntropy)
}
pub fn secure_random_u64() -> CryptoResult<u64> {
    let mut bytes = [0u8; 8];
    fill_random(&mut bytes)?;
    Ok(u64::from_le_bytes(bytes))
}
pub fn secure_random_u32() -> CryptoResult<u32> {
    Ok(secure_random_u64()? as u32)
}
pub fn secure_random_u8() -> CryptoResult<u8> {
    Ok(secure_random_u64()? as u8)
}

pub fn generate_secure_key_checked() -> Result<[u8; 32], &'static str> {
    let mut key = [0u8; 32];
    rng::fill_random_bytes_secure(&mut key)
        .map_err(|_| "Insufficient entropy for key generation")?;
    Ok(key)
}
