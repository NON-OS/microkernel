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

//! The head of a PUT that sends a body of known length.

use alloc::vec::Vec;

use crate::term::util::format_u64;

pub fn build_put_head(host: &[u8], path: &[u8], len: usize, extra: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(b"PUT ");
    r.extend_from_slice(path);
    r.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    r.extend_from_slice(host);
    r.extend_from_slice(b"\r\nConnection: close\r\nUser-Agent: nonos-nox\r\nContent-Length: ");
    let mut buf = [0u8; 24];
    let k = format_u64(len as u64, &mut buf);
    r.extend_from_slice(&buf[..k]);
    r.extend_from_slice(b"\r\n");
    r.extend_from_slice(extra);
    r.extend_from_slice(b"\r\n");
    r
}
