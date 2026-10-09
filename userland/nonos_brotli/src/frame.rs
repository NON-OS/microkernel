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

/* Stream framing: the window size up front, and the byte-aligned data
of stored and metadata blocks. */

use crate::bits::Bits;
use crate::error::Error;

/// log2 of the sliding window size (RFC 7932 9.1).
pub(crate) fn window_bits(b: &mut Bits) -> Result<u32, Error> {
    if b.read(1)? == 0 {
        return Ok(16);
    }
    match b.read(3)? {
        0 => match b.read(3)? {
            0 => Ok(17),
            1 => Err(Error::Invalid),
            m => Ok(8 + m),
        },
        n => Ok(17 + n),
    }
}

/// Skip to a byte boundary through zero bits, then hand `len` whole
/// bytes to `out`: buffered ones first, the rest straight from input.
pub(crate) fn bytes(b: &mut Bits, len: usize, mut out: impl FnMut(u8)) -> Result<(), Error> {
    if b.read(b.buffered() % 8)? != 0 {
        return Err(Error::Invalid);
    }
    let mut left = len;
    while left > 0 && b.buffered() >= 8 {
        out(b.read(8)? as u8);
        left -= 1;
    }
    b.raw(left).ok_or(Error::Truncated)?.iter().for_each(|&x| out(x));
    Ok(())
}
