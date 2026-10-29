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

//! The server's WebSocket frames, read out of the bytes the stream has
//! brought so far.

use alloc::string::String;
use alloc::vec::Vec;

use super::ws_frame::{CLOSE, PING, TEXT};

#[derive(Debug, PartialEq, Eq)]
pub enum Got {
    Text(String),
    Ping(Vec<u8>),
    Closed,
    Other,
}

/// The first whole frame in `buf`, taken out of it; None until one has come.
pub fn take(buf: &mut Vec<u8>) -> Option<Got> {
    let (op, masked, short) = (*buf.first()? & 0x0f, buf.get(1)? & 0x80 != 0, buf[1] & 0x7f);
    let (len, at) = match short {
        126 => (u16::from_be_bytes([*buf.get(2)?, *buf.get(3)?]) as usize, 4),
        127 => (u64::from_be_bytes(buf.get(2..10)?.try_into().ok()?) as usize, 10),
        n => (n as usize, 2),
    };
    let key: [u8; 4] = if masked { buf.get(at..at + 4)?.try_into().ok()? } else { [0; 4] };
    let start = at + if masked { 4 } else { 0 };
    let end = start.checked_add(len)?;
    let data: Vec<u8> =
        buf.get(start..end)?.iter().enumerate().map(|(i, b)| b ^ key[i % 4]).collect();
    buf.drain(..end);
    Some(match op {
        TEXT => String::from_utf8(data).map_or(Got::Other, Got::Text),
        PING => Got::Ping(data),
        CLOSE => Got::Closed,
        _ => Got::Other,
    })
}
