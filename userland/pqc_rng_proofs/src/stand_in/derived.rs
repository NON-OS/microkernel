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

//! The draws the samplers make from the stand-in's words: a word cut to 32
//! bits, and a value below a bound by rejection, as the kernel's.

use super::next;

pub(crate) fn random_u32_secure() -> Result<u32, ()> {
    Ok(next()? as u32)
}

pub(crate) fn random_range_secure(n: u32) -> Result<u32, ()> {
    if n <= 1 {
        return Ok(0);
    }
    let threshold = u32::MAX - (u32::MAX % n);
    loop {
        let r = random_u32_secure()?;
        if r < threshold {
            return Ok(r % n);
        }
    }
}
