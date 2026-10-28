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

//! The WebSocket opening handshake (RFC 6455, section 4) the window makes to
//! the core's chat API: one GET asking to upgrade, and the server's answer,
//! which must be 101 Switching Protocols. The bytes after the answer's blank
//! line are already frames.

use alloc::format;
use alloc::string::String;

/// The upgrade request, with a key of sixteen bytes in base64.
pub fn request(key: &[u8; 16]) -> String {
    format!(
        "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n         Sec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n\r\n",
        base64(key)
    )
}

pub enum Answer {
    /// Not all of the answer's head has come yet.
    More,
    /// Upgraded; the head took this many bytes.
    Upgraded(usize),
    /// Anything but 101.
    Refused,
}

/// Reads the server's answer from what has come so far.
pub fn answer(got: &[u8]) -> Answer {
    let Some(end) = got.windows(4).position(|w| w == b"\r\n\r\n") else {
        return if got.len() > 4096 { Answer::Refused } else { Answer::More };
    };
    let line = got.split(|&b| b == b'\n').next().unwrap_or_default();
    match line.starts_with(b"HTTP/1.1 101") {
        true => Answer::Upgraded(end + 4),
        false => Answer::Refused,
    }
}

fn base64(bytes: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for c in bytes.chunks(3) {
        let n = (u32::from(c[0]) << 16)
            | (u32::from(*c.get(1).unwrap_or(&0)) << 8)
            | u32::from(*c.get(2).unwrap_or(&0));
        for i in 0..4 {
            match i <= c.len() {
                true => out.push(A[(n >> (18 - 6 * i)) as usize & 63] as char),
                false => out.push('='),
            }
        }
    }
    out
}
