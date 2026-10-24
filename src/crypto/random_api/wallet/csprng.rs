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

use super::pool::EntropyPool;
use crate::crypto::error::{CryptoError, CryptoResult};
use crate::crypto::util::rng;

/// The seeded generator's share of the pool; refused before it is seeded,
/// since the pool's other sources are timing and system state.
pub(super) fn collect(pool: &mut EntropyPool) -> CryptoResult<()> {
    let mut rng_bytes = [0u8; 64];
    rng::fill_random_bytes_secure(&mut rng_bytes).map_err(|_| CryptoError::InsufficientEntropy)?;
    pool.append(&rng_bytes);
    zeroize(&mut rng_bytes);
    Ok(())
}

fn zeroize(bytes: &mut [u8]) {
    for byte in bytes {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
}
