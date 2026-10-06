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

use alloc::borrow::Cow;

use crate::browser::http::chunked;

use super::codings::Coding;
use super::head::{Framing, Head};
use super::undo_coding::undo;

/* The body of the final response with its transfer and content codings
undone (last applied first), and whether all of it arrived. With
`partial`, a body cut short by its framing or its coding comes back
as far as it goes; without, only a complete body does. Malformed
framing or coding, or a coding this browser lacks, is None. */
pub fn decode_body<'a>(raw: &'a [u8], h: &Head, partial: bool) -> Option<(Cow<'a, [u8]>, bool)> {
    if h.framing == Framing::Empty {
        return Some((Cow::Borrowed(&raw[..0]), true));
    }
    if h.content.undecodable || h.transfer.undecodable {
        return None;
    }
    let (mut body, mut complete) = match h.framing {
        Framing::Empty => (Cow::Borrowed(&raw[..0]), true),
        Framing::Length(n) => {
            let n = usize::try_from(n).unwrap_or(usize::MAX);
            (Cow::Borrowed(&raw[..n.min(raw.len())]), raw.len() >= n)
        }
        Framing::Chunked if partial => {
            let (b, done) = chunked::decode_partial(raw)?;
            (Cow::Owned(b), done)
        }
        Framing::Chunked => (Cow::Owned(chunked::decode(raw)?), true),
        Framing::Close => (Cow::Borrowed(raw), true),
    };
    let te = h.transfer.as_slice();
    let te = if h.framing == Framing::Chunked { &te[..te.len() - 1] } else { te };
    for &c in te.iter().rev().chain(h.content.as_slice().iter().rev()) {
        if c == Coding::Chunked {
            return None;
        }
        let (out, done) = undo(c, &body, partial)?;
        body = Cow::Owned(out);
        complete &= done;
    }
    (complete || partial).then_some((body, complete))
}
