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

//! Whose a conversation is: the caller the kernel attests, and the stream
//! that caller named.
//!
//! A conversation was keyed on the caller's pid alone, so a program held one
//! at a time and a browser fetched a page's resources one after another, a
//! SOCKS handshake and an Anyone stream each. A frame may now name a stream
//! (`frame`), and each stream is a conversation of its own. The pid stays
//! half of the key and is the kernel's word, never the caller's, so no
//! caller reaches a stream of another's whatever number it names. A frame
//! that names none is stream 0, the one conversation every caller had.
//! net.socks5 keys its conversations the same way (its server/who.rs).

/// A pid in the high half, the stream it named in the low.
pub type Who = u64;

/// The stream of a frame that names none.
pub const UNNAMED: u32 = 0;

/// Conversations one caller may hold at once, of the CALLERS_MAX all share.
pub const STREAMS_PER_CALLER: usize = 8;

pub fn who(pid: u32, stream: u32) -> Who {
    (u64::from(pid) << 32) | u64::from(stream)
}

/// The caller a conversation belongs to.
pub fn pid_of(w: Who) -> u32 {
    (w >> 32) as u32
}
