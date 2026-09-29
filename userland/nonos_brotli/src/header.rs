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

use crate::bits::Bits;
use crate::error::Error;

/// What a meta-block holds after its header (RFC 7932 9.2).
pub(crate) enum Body {
    /// Nothing to output; that many bytes of metadata follow.
    Metadata(usize),
    /// That many bytes, stored as they are.
    Stored(usize),
    /// That many bytes, coded as commands.
    Compressed(usize),
}

/// The header of a meta-block: whether it is the last, and its body.
pub(crate) fn meta_header(b: &mut Bits) -> Result<(bool, Body), Error> {
    let last = b.read(1)? == 1;
    if last && b.read(1)? == 1 {
        return Ok((true, Body::Metadata(0)));
    }
    let nibbles = b.read(2)? as usize;
    if nibbles == 3 {
        if b.read(1)? != 0 {
            return Err(Error::Invalid);
        }
        let bytes = b.read(2)? as usize;
        return Ok((last, Body::Metadata(size(b, bytes, 8, 1)?)));
    }
    let len = size(b, nibbles + 4, 4, 4)?;
    match !last && b.read(1)? == 1 {
        true => Ok((last, Body::Stored(len))),
        false => Ok((last, Body::Compressed(len))),
    }
}

/// A length sent as `count` digits of `bits` bits, minus one; a top
/// digit of zero is refused when more than `min` digits are sent.
fn size(b: &mut Bits, count: usize, bits: u32, min: usize) -> Result<usize, Error> {
    let mut v = 0usize;
    for i in 0..count {
        let d = b.read(bits)? as usize;
        if d == 0 && i + 1 == count && count > min {
            return Err(Error::Invalid);
        }
        v |= d << (i as u32 * bits);
    }
    Ok(if count == 0 { 0 } else { v + 1 })
}

/// A count of block types or trees, 1..=256 (RFC 7932 9.2).
pub(crate) fn read_count(b: &mut Bits) -> Result<usize, Error> {
    if b.read(1)? == 0 {
        return Ok(1);
    }
    Ok(match b.read(3)? {
        0 => 2,
        n => (1 << n) + b.read(n)? as usize + 1,
    })
}
