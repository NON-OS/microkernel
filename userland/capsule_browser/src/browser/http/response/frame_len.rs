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

use crate::browser::http::chunked;

use super::head::{scan, Framing, Scan};

/* Total bytes the response at the head of `raw` occupies, interim
responses and headers included, once it is complete. On a kept-alive
connection this is where the next response begins. None until the
framing is satisfied, when the response is close-delimited, and for
any head parse refuses, so framing and parsing never disagree. */
pub fn frame_len(raw: &[u8]) -> Option<usize> {
    let Scan::Head(h) = scan(raw) else { return None };
    let body = &raw[h.body_at..];
    let n = match h.framing {
        Framing::Empty => 0,
        Framing::Length(n) => usize::try_from(n).ok().filter(|&n| n <= body.len())?,
        Framing::Chunked => chunked::frame_end(body)?,
        Framing::Close => return None,
    };
    h.body_at.checked_add(n)
}
