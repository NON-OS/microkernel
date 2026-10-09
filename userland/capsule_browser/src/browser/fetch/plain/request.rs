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

use crate::browser::fetch::types::Fetch;
use crate::browser::http;

/*
 * A GET asks to keep the connection whenever the fetch may be kept, so the
 * next request to the same host skips the connect and, over TLS, the
 * handshake; through a network's proxy it also skips the SOCKS handshake
 * and the tunnel. A POST, and anything over a SOCKS proxy the reader set,
 * asks to close.
 */
/// The request `f` sends, with the cookies its network's jar holds for its
/// address at the wall clock `now_ms`.
pub(in crate::browser::fetch) fn request(f: &Fetch, now_ms: i64) -> String {
    let now = crate::browser::cookie::unix_secs(now_ms);
    let cookie = crate::browser::cookie::request_header(f.net(), &f.url, now);
    match f.post.as_deref() {
        None if f.keep => http::request::build_keep_alive(&f.url, cookie.as_deref()),
        post => http::request::build(&f.url, post, cookie.as_deref()),
    }
}
