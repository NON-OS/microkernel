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

//! The kernel's CSPRNG, the one source of every key, nonce and signing
//! randomness nsh.attest uses. A fill that does not complete is a failure,
//! never a short key.

use nonos_libc::crypto_random;

/// The most one kernel call fills.
const CALL_MAX: usize = 4096;

/// Fill `out` with fresh random bytes; `false` if the kernel could not.
pub fn fill(out: &mut [u8]) -> bool {
    out.chunks_mut(CALL_MAX).all(|chunk| {
        let rc = crypto_random(chunk.as_mut_ptr(), chunk.len());
        rc >= 0 && rc as usize == chunk.len()
    })
}
