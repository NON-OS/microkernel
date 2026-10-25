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

//! The kernel's `random_api::get_bytes_secure`, over the proof set's RNG
//! stand-in: its bytes, or `InsufficientEntropy` once a test has taken the
//! generator away, as the kernel's refuses before the RNG is seeded.

use super::error::{CryptoError, CryptoResult};

pub fn get_bytes_secure(buf: &mut [u8]) -> CryptoResult<()> {
    for chunk in buf.chunks_mut(32) {
        let bytes =
            super::rng::get_random_bytes_secure().map_err(|_| CryptoError::InsufficientEntropy)?;
        chunk.copy_from_slice(&bytes[..chunk.len()]);
    }
    Ok(())
}

pub fn get_vec_secure(len: usize) -> CryptoResult<alloc::vec::Vec<u8>> {
    let mut out = alloc::vec![0u8; len];
    get_bytes_secure(&mut out)?;
    Ok(out)
}
