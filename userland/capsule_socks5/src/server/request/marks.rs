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

//! The marker bytes that open every request to the proxy.

/// Whatever follows is stream bytes for the tunnel, of which there may be
/// none.
pub const STREAM_BYTES: u8 = 0;

/// Forget whatever conversation this caller had and start over.
pub const STREAM_RESET: u8 = 1;

/// Stream bytes carried in a numbered exchange: the marker, a u32 number
/// (little endian), then the bytes. See `kept` for why.
pub const STREAM_NUMBERED: u8 = 2;

/// A reset naming its stream: the marker, then a u32 stream (little endian,
/// never 0). See `who` for why a caller names streams.
pub const STREAM_RESET_ON: u8 = 3;

/// A numbered exchange naming its stream: the marker, a u32 stream (never
/// 0), a u32 number, then the bytes. Both numbers little endian.
pub const STREAM_NUMBERED_ON: u8 = 4;

/// How far the proxy has got with its network: the marker alone. Answered
/// with `reply::progress`, never with stream bytes, and it touches no
/// conversation, so a caller may ask it while a CONNECT waits. A proxy older
/// than this marker answers it as a reset of stream 0, which the browser,
/// the one caller that asks, never uses. Not 5: that is the SOCKS version,
/// the first byte of a greeting sent without a marker.
pub const STATUS_ASK: u8 = 6;
