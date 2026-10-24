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
//! The security-key wire, NFID version 1: the host wire's 20-byte header
//! with its own magic (op u16 at 6, request id u32 at 12, payload length
//! u32 at 16), then the body; a reply's payload starts with its status.

pub const MAGIC: u32 = 0x4E46_4944; // "NFID"
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_HEALTHCHECK: u16 = 1;
/// The key taken: `vendor u16, product u16, caps u8, major u8, minor u8,
/// build u8`.
pub const OP_OPEN: u16 = 2;
/// A running request cancelled and drained, the key let go.
pub const OP_CLOSE: u16 = 3;
/// One CTAP2 request, its command byte and CBOR unread: 0 once sent.
pub const OP_CBOR: u16 = 4;
/// The answer, status byte and CBOR as the key gave them; -11 with the
/// keep-alive status while it runs.
pub const OP_RESULT: u16 = 5;
/// CTAPHID_CANCEL for the running request; its answer still comes.
pub const OP_CANCEL: u16 = 6;

pub const CHUNK: usize = 4000;
pub const REPLY_MAX: usize = HDR_LEN + 4 + CHUNK;

pub const E_IO: i32 = -5;
pub const E_BUSY: i32 = -16;
pub const E_NODEV: i32 = -19;
pub const E_INVAL: i32 = -22;
/// An answer longer than a reply carries.
pub const E_MSGSIZE: i32 = -90;
