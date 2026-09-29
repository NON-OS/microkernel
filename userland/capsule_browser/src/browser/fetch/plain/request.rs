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
 * handshake. A POST, and anything over a proxy or the mixnet, asks to close.
 */
/// The request `f` sends.
pub(in crate::browser::fetch) fn request(f: &Fetch) -> String {
    match f.post.as_deref() {
        None if f.keep => http::request::build_keep_alive(&f.url),
        post => http::request::build(&f.url, post),
    }
}
