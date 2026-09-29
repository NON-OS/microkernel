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

use alloc::string::String;

use super::codings::Codings;
use super::mime::Mime;

/* How the body of the final response is delimited (RFC 9112 6.3). */
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Framing {
    /* 1xx, 204 and 304 have no content, whatever the fields say. */
    Empty,
    Length(u64),
    Chunked,
    /* Read until the server closes. */
    Close,
}

/* Everything the fetch path uses from a response head. */
pub struct Head {
    pub status: u16,
    /* Offset of the body in the raw bytes, interim responses included. */
    pub body_at: usize,
    pub framing: Framing,
    pub content: Codings,
    pub transfer: Codings,
    pub mime: Mime,
    pub location: Option<String>,
    /* The server will close the connection after this response. */
    pub close: bool,
}

pub enum Scan {
    /* The final response's header section has not all arrived. */
    More,
    /* It arrived and is not a response this browser can frame. */
    Bad,
    Head(Head),
}
