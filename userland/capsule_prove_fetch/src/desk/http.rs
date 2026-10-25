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

//! What fetch says to the relay, a content-addressed store keyed by anchor
//! and trusted for nothing: `PUT` or `GET /v1/proof/<anchor hex>`, HTTP/1.1
//! over TLS, the container as the body, the connection closed after.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub fn path(anchor: &[u8; 32]) -> String {
    let hex: String = anchor.iter().map(|b| format!("{b:02x}")).collect();
    format!("/v1/proof/{hex}")
}

pub fn put_request(host: &str, anchor: &[u8; 32], body: &[u8]) -> Vec<u8> {
    let head = format!(
        "PUT {} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/octet-stream\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n",
        path(anchor),
        body.len()
    );
    [head.as_bytes(), body].concat()
}

pub fn get_request(host: &str, anchor: &[u8; 32]) -> Vec<u8> {
    format!("GET {} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n", path(anchor))
        .into_bytes()
}
