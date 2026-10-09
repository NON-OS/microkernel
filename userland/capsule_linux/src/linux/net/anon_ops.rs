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

//! What this capsule speaks to net.anon's handle front. Each value is kept
//! in sync with userland/capsule_net_anon/src/protocol/ (ops.rs, errno.rs,
//! limits.rs) and src/stream/ (end.rs, table.rs); capsule_linux_proofs
//! holds every one to the file it comes from.

/// The request and reply header: magic u32, version u16, op u16, status
/// u16, two bytes of zero, request id u32, body length u32, little-endian.
pub const MAGIC: u32 = 0x414E_4F31;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

/// The most one request or reply carries past the header.
pub const PAYLOAD_MAX: usize = 32 * 1024;

/// Body: port u16, then the host. Reply: the stream id, u16.
pub const OP_OPEN_STREAM: u16 = 5;
/// Body: stream id u16, then the bytes. Reply: the count taken, u32.
pub const OP_SEND: u16 = 6;
/// Body: stream id u16. Reply: the bytes that have arrived.
pub const OP_RECV: u16 = 7;
/// Body: stream id u16.
pub const OP_CLOSE_STREAM: u16 = 8;

pub const E_OK: u16 = 0;
pub const E_BAD_LEN: u16 = 4;
pub const E_NO_TCP: u16 = 5;
pub const E_NO_DIRECTORY: u16 = 6;
pub const E_NO_PATH: u16 = 7;
pub const E_NO_LINK: u16 = 8;
pub const E_TABLE_FULL: u16 = 9;
pub const E_NO_CIRCUIT: u16 = 10;
pub const E_NO_STREAM: u16 = 11;
pub const E_RX_EMPTY: u16 = 14;
/// Body, three bytes: the END reason, whether another exit is needed,
/// whether it was clean.
pub const E_STREAM_CLOSED: u16 = 15;
pub const E_DIRECTORY_STALE: u16 = 18;
pub const E_WOULD_BLOCK: u16 = 20;

/// END reasons, tor-spec 6.3: the far end finished, its circuit was torn
/// down, the exit gave up waiting for the host.
pub const REASON_DONE: u8 = 6;
pub const REASON_DESTROY: u8 = 5;
pub const REASON_TIMEOUT: u8 = 7;
