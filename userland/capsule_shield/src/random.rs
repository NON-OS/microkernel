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

//! The randomness every crate asks for: getrandom's custom backend is the
//! kernel's CSPRNG. A short draw is an error, never zeros.

use core::num::NonZeroU32;

/// A short draw from the kernel, as getrandom's first code for a custom backend.
const SHORT_DRAW: NonZeroU32 = match NonZeroU32::new(getrandom02::Error::CUSTOM_START) {
    Some(code) => code,
    None => NonZeroU32::MIN,
};

fn fill(dest: *mut u8, len: usize) -> bool {
    len == 0 || nonos_libc::crypto_random(dest, len) == len as i64
}

/// The most the kernel draws in one call: a larger ask is refused whole.
const DRAW_MAX: usize = 4096;

fn v02(buf: &mut [u8]) -> Result<(), getrandom02::Error> {
    for chunk in buf.chunks_mut(DRAW_MAX) {
        if !fill(chunk.as_mut_ptr(), chunk.len()) {
            return Err(getrandom02::Error::from(SHORT_DRAW));
        }
    }
    Ok(())
}

getrandom02::register_custom_getrandom!(v02);
