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

use nonos_inflate::{End, Inflated, MAX_OUT};

use super::codings::Coding;

/* A body cut at the output cap keeps its head only while the cut has
read at least this share (1/16) of the coded bytes; below that the
rest would inflate past sixteen times the cap, which is a
decompression bomb, not a page. */
const MIN_READ_SHARE: usize = 16;

/* One content or transfer coding undone, its output capped at `MAX_OUT`:
the output and whether the coded stream was complete. A stream that
stops short, or reaches the cap after reading enough of its input,
passes when `partial` allows; a corrupt one never does. */
pub fn undo(c: Coding, data: &[u8], partial: bool) -> Option<(Vec<u8>, bool)> {
    let r = match c {
        Coding::Gzip => nonos_inflate::gunzip_partial(data, MAX_OUT),
        Coding::Deflate => deflate(data),
        Coding::Chunked => return None,
    };
    let kept = match r.end {
        End::Complete => return Some((r.out, true)),
        End::Truncated => partial,
        End::Capped => partial && r.used.saturating_mul(MIN_READ_SHARE) >= data.len(),
        End::Corrupt => false,
    };
    kept.then_some((r.out, false))
}

/* "deflate" is zlib-wrapped by the standard, but some servers send the
raw DEFLATE stream; browsers take either. */
fn deflate(data: &[u8]) -> Inflated {
    let z = nonos_inflate::zlib_partial(data, MAX_OUT);
    if z.end != End::Corrupt {
        return z;
    }
    let raw = nonos_inflate::raw_partial(data, MAX_OUT);
    if raw.end == End::Corrupt {
        z
    } else {
        raw
    }
}
