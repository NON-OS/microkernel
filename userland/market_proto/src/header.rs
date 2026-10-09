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

//! The 20-byte header both directions carry.
//!
//!   bytes  0..4   magic "NMKT", as a little-endian word
//!          4..6   version
//!          6..8   op (a reply echoes the request's)
//!          8..10  flags (echoed)
//!         10..12  reserved, zero
//!         12..16  request id (echoed)
//!         16..20  payload length: the bytes after the header
//!
//! A reply's payload starts with a four-byte status, zero for success.

pub const MAGIC: u32 = 0x4E4D_4B54;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
