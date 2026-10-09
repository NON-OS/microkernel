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

use alloc::vec::Vec;

use super::line::line_end;
use super::parse_hex::parse_hex;

/* Where a chunked body stands after the bytes received so far. */
pub enum Walk {
    /* The last chunk and the trailer section ended this many bytes in. */
    Done(usize),
    /* The bytes stop before the body does; what arrived is well formed. */
    Short,
    /* A size line or a chunk's line ending is malformed. */
    Bad,
}

/* The one reading of chunked framing (RFC 9112 7.1) that completion,
keep-alive framing and decoding share. Chunk data goes to `out` when
given, including the received part of a chunk the bytes cut short. */
pub fn walk(body: &[u8], mut out: Option<&mut Vec<u8>>) -> Walk {
    let mut i = 0usize;
    loop {
        let Some((end, next)) = line_end(&body[i..]) else { return Walk::Short };
        let Some(size) = parse_hex(&body[i..i + end]) else { return Walk::Bad };
        i += next;
        if size == 0 {
            return trailers(body, i);
        }
        let data = &body[i..];
        if let Some(o) = out.as_deref_mut() {
            o.extend_from_slice(&data[..size.min(data.len())]);
        }
        if data.len() < size {
            return Walk::Short;
        }
        i += size;
        match &body[i..] {
            [] | [b'\r'] => return Walk::Short,
            [b'\n', ..] => i += 1,
            [b'\r', b'\n', ..] => i += 2,
            _ => return Walk::Bad,
        }
    }
}

/* Trailer fields after the last chunk, up to the empty line. */
fn trailers(body: &[u8], mut i: usize) -> Walk {
    loop {
        let Some((end, next)) = line_end(&body[i..]) else { return Walk::Short };
        i += next;
        if end == 0 {
            return Walk::Done(i);
        }
    }
}
