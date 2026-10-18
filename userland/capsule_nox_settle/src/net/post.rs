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

//! A POST of raw bytes to an https url's own path, for FETCH 1 ESCROW
//! (plan/22 step 1): the same route, TLS and limits as the release fetch, and
//! no redirect followed.

use alloc::vec::Vec;

use super::https::{https, FETCH_TOTAL_MS};

pub fn post_to(host: &str, path: &str, body: &[u8], limit: usize) -> Result<Vec<u8>, i32> {
    let mut req = Vec::with_capacity(
        host.len().saturating_add(path.len()).saturating_add(body.len()).saturating_add(160),
    );
    for part in ["POST ", path, " HTTP/1.1\r\nHost: ", host] {
        req.extend_from_slice(part.as_bytes());
    }
    req.extend_from_slice(b"\r\nContent-Type: application/octet-stream\r\nContent-Length: ");
    req.extend_from_slice(alloc::format!("{}", body.len()).as_bytes());
    req.extend_from_slice(b"\r\nConnection: close\r\n\r\n");
    req.extend_from_slice(body);
    https(host, &req, limit, FETCH_TOTAL_MS)
}
