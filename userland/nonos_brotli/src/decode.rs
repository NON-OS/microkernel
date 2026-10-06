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
use crate::distance::Ring;
use crate::error::Error;
use crate::frame::{bytes, window_bits};
use crate::header::{meta_header, Body};
use crate::metablock::read_meta;
use alloc::vec::Vec;

/// Decompress a whole Brotli stream (RFC 7932). An output that would
/// pass `max_out` bytes is refused with `TooLarge` before it is built.
/// Input bytes after the end of the stream are left unread, as the
/// reference decoder leaves them.
pub fn decompress(input: &[u8], max_out: usize) -> Result<Vec<u8>, Error> {
    let mut b = Bits::new(input);
    let window = (1usize << window_bits(&mut b)?) - 16;
    let (mut out, mut ring) = (Vec::new(), Ring::new());
    loop {
        let (last, body) = meta_header(&mut b)?;
        match body {
            Body::Metadata(skip) => bytes(&mut b, skip, |_| {})?,
            Body::Stored(len) => {
                grow(&mut out, len, max_out)?;
                bytes(&mut b, len, |x| out.push(x))?;
            }
            Body::Compressed(len) => {
                grow(&mut out, len, max_out)?;
                let mut meta = read_meta(&mut b)?;
                crate::run::run(&mut b, &mut meta, &mut out, (len, window), &mut ring)?;
            }
        }
        if last {
            bytes(&mut b, 0, |_| {})?;
            return Ok(out);
        }
    }
}

/// Make room for `len` more bytes, within the cap.
fn grow(out: &mut Vec<u8>, len: usize, max_out: usize) -> Result<(), Error> {
    if len > max_out - out.len() {
        return Err(Error::TooLarge);
    }
    out.try_reserve(len).map_err(|_| Error::NoMemory)
}
