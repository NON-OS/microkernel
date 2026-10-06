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


//! The Wayland message header and the argument cursor.

pub const HEADER: usize = 8;

pub struct Msg<'a> {
    pub object: u32,
    pub opcode: u16,
    pub args: &'a [u8],
}

/// The next message in `buf`, or nothing if it has not all arrived.
/// True when `buf` starts with a header whose size is less than a header,
/// which no later byte can make whole.
pub fn malformed(buf: &[u8]) -> bool {
    buf.len() >= HEADER && (u16::from_le_bytes([buf[6], buf[7]]) as usize) < HEADER
}

pub fn next(buf: &[u8]) -> Option<(Msg<'_>, usize)> {
    if buf.len() < HEADER {
        return None;
    }
    let object = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let opcode = u16::from_le_bytes([buf[4], buf[5]]);
    let size = u16::from_le_bytes([buf[6], buf[7]]) as usize;
    if size < HEADER || size > buf.len() {
        return None;
    }
    Some((Msg { object, opcode, args: &buf[HEADER..size] }, size))
}

/// Each whole message at the front of `buf`, in order, to `each`, until it
/// answers false or no whole message is left; the bytes walked, for the
/// caller to cut from its queue once. Each message is walked past before
/// it is handed on, so one a handler refuses is never handed on again.
///
/// Cut from the queue one message at a time, every message moved all the
/// bytes behind it, and a mebibyte of 12-byte requests in one write moved
/// some forty gigabytes before the write was answered.
pub fn walk<'a>(buf: &'a [u8], mut each: impl FnMut(Msg<'a>) -> bool) -> usize {
    let mut at = 0;
    while let Some((msg, size)) = buf.get(at..).and_then(next) {
        at += size;
        if !each(msg) {
            break;
        }
    }
    at
}
