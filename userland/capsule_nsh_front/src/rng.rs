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

//! The entropy the server signs with. `nsh_server` reads no randomness of
//! its own; every signature's nonce and every session id comes through here,
//! from the kernel's CSPRNG. A fill that does not complete says so, and the
//! server fails the call closed rather than signing with weak randomness.

use nonos_libc::crypto_random;

/// Fill `out` with fresh random bytes; `false` if the kernel could not.
pub fn fill(out: &mut [u8]) -> bool {
    let rc = crypto_random(out.as_mut_ptr(), out.len());
    rc >= 0 && rc as usize == out.len()
}
