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

use super::rng::{random_u64_secure, RngResult};

/// Draws only from the seeded RNG, and fails before it is seeded.
pub fn secure_random_u64() -> RngResult<u64> {
    random_u64_secure()
}

pub fn secure_random_u32() -> RngResult<u32> {
    Ok((secure_random_u64()? >> 32) as u32)
}

pub fn secure_zero(buffer: &mut [u8]) {
    for byte in buffer.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
