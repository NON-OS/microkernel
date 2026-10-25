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

//! The agent's HTTP, one request per connection: a POST whose
//! Content-Length is exactly BODY_LEN, and nothing longer is read. TLS is
//! the operator's front (the release's package_url is https); this speaks
//! plain HTTP to it on a local address.

use std::io::{Read, Write};

use super::body::BODY_LEN;

const HEAD_MAX: usize = 4096;

/// The request's body, or None for anything but a POST of BODY_LEN bytes.
pub fn read_request(s: &mut impl Read) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(HEAD_MAX);
    let mut byte = [0u8; 1];
    while !buf.ends_with(b"\r\n\r\n") {
        (buf.len() < HEAD_MAX && s.read(&mut byte).ok()? == 1).then_some(())?;
        buf.push(byte[0]);
    }
    let head = std::str::from_utf8(&buf).ok()?.to_ascii_lowercase();
    head.starts_with("post ").then_some(())?;
    let len = head.lines().find_map(|l| l.strip_prefix("content-length:"))?;
    (len.trim().parse::<usize>().ok()? == BODY_LEN).then_some(())?;
    let mut body = vec![0u8; BODY_LEN];
    s.read_exact(&mut body).ok()?;
    Some(body)
}

pub fn write_answer(s: &mut impl Write, status: u16, body: &[u8]) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        429 => "Too Many Requests",
        _ => "Bad Request",
    };
    write!(
        s,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/octet-stream\r\n         Content-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    s.write_all(body)?;
    s.flush()
}
