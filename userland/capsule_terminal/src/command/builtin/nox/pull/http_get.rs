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

//! The request heads that read: a GET that closes, a GET that keeps the connection, a HEAD.

use alloc::vec::Vec;

pub fn build_get(host: &[u8], path: &[u8], extra: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(b"GET ");
    r.extend_from_slice(path);
    r.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    r.extend_from_slice(host);
    r.extend_from_slice(
        b"\r\nConnection: close\r\nUser-Agent: nonos-nox\r\nAccept-Encoding: gzip\r\n",
    );
    r.extend_from_slice(extra);
    r.extend_from_slice(b"\r\n");
    r
}

pub fn build_get_ka(host: &[u8], path: &[u8], extra: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(b"GET ");
    r.extend_from_slice(path);
    r.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    r.extend_from_slice(host);
    r.extend_from_slice(
        b"\r\nConnection: keep-alive\r\nUser-Agent: nonos-nox\r\nAccept-Encoding: gzip\r\n",
    );
    r.extend_from_slice(extra);
    r.extend_from_slice(b"\r\n");
    r
}

pub fn build_head(host: &[u8], path: &[u8], extra: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(b"HEAD ");
    r.extend_from_slice(path);
    r.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    r.extend_from_slice(host);
    r.extend_from_slice(b"\r\nConnection: keep-alive\r\nUser-Agent: nonos-nox\r\n");
    r.extend_from_slice(extra);
    r.extend_from_slice(b"\r\n");
    r
}
