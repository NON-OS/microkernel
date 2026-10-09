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

/* True once no further byte can change what parse makes of `raw`: the
final response's body is all there, or its head is one no body can
repair. A close-delimited body is never known to be complete here. */
pub fn is_complete(raw: &[u8]) -> bool {
    let h = match scan(raw) {
        Scan::More => return false,
        Scan::Bad => return true,
        Scan::Head(h) => h,
    };
    let body = &raw[h.body_at..];
    match h.framing {
        Framing::Empty => true,
        Framing::Length(n) => body.len() as u64 >= n,
        Framing::Chunked => chunked::complete(body),
        Framing::Close => false,
    }
}
