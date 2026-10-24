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

//! A report that opens something (bit 7 of its command byte set), judged
//! before it touches a message (CTAP 2.x, 11.2.4 and 11.2.9, as recalled,
//! unverified). KEEPALIVE carries one byte, status 1 (the key is working)
//! or 2 (it waits for its user's touch); another status or length is let
//! go. ERROR carries one byte, the key's code, passed up as it came; one
//! of another length ends the message as broken.

use super::frame::{CMD_ERROR, CMD_KEEPALIVE, MESSAGE_MAX};

const PROCESSING: u8 = 1;
const UP_NEEDED: u8 = 2;

pub enum Opening {
    Keepalive(u8),
    Error(u8),
    /// A message of this command and length begins.
    Begin(u8, usize),
    Ignore,
    Broken,
}

/// `p` is at least seven bytes: channel, command, length.
pub fn opening(p: &[u8]) -> Opening {
    let bcnt = u16::from_be_bytes([p[5], p[6]]) as usize;
    match (p[4], bcnt, p.get(7).copied()) {
        (CMD_KEEPALIVE, 1, Some(s @ (PROCESSING | UP_NEEDED))) => Opening::Keepalive(s),
        (CMD_KEEPALIVE, ..) => Opening::Ignore,
        (CMD_ERROR, 1, Some(code)) => Opening::Error(code),
        (CMD_ERROR, ..) => Opening::Broken,
        // Past one opening and 128 continuations of 64-byte reports.
        (_, n, _) if n > MESSAGE_MAX => Opening::Broken,
        (cmd, n, _) => Opening::Begin(cmd, n),
    }
}
