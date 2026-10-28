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

//! WebSocket frames (RFC 6455, section 5) as a client writes them: whole,
//! and masked, as a client's must be.

use alloc::vec::Vec;

pub const TEXT: u8 = 1;
pub const CLOSE: u8 = 8;
pub const PING: u8 = 9;
pub const PONG: u8 = 10;

/// One whole text frame carrying `text`, masked with `mask`.
pub fn text(text: &str, mask: [u8; 4]) -> Vec<u8> {
    frame(TEXT, text.as_bytes(), mask)
}

/// The pong a ping asks for, with its data.
pub fn pong(data: &[u8], mask: [u8; 4]) -> Vec<u8> {
    frame(PONG, data, mask)
}

fn frame(op: u8, data: &[u8], mask: [u8; 4]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + 14);
    out.push(0x80 | op);
    match data.len() {
        n if n < 126 => out.push(0x80 | n as u8),
        n if n <= 0xffff => {
            out.push(0x80 | 126);
            out.extend_from_slice(&(n as u16).to_be_bytes());
        }
        n => {
            out.push(0x80 | 127);
            out.extend_from_slice(&(n as u64).to_be_bytes());
        }
    }
    out.extend_from_slice(&mask);
    out.extend(data.iter().enumerate().map(|(i, b)| b ^ mask[i % 4]));
    out
}
