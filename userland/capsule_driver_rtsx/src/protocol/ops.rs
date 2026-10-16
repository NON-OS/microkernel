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

//! The ops and their bodies. Every count is in 512-byte blocks.
//!
//! - HEALTHCHECK, CAPACITY, FLUSH and IDENTIFY take no body.
//! - CAPACITY answers the card's size in blocks, u64 little endian.
//! - READ takes lba (u64 LE) and count (u32 LE) and answers the blocks.
//! - WRITE takes the same twelve bytes and then the blocks; it answers
//!   once the card has programmed them.
//! - FLUSH answers once the card has programmed everything it was sent.
//! - IDENTIFY answers the layout in identify.rs.

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_CAPACITY: u16 = 2;
pub const OP_READ: u16 = 3;
pub const OP_WRITE: u16 = 4;
pub const OP_FLUSH: u16 = 5;
pub const OP_IDENTIFY: u16 = 6;

pub const RW_HEADER_LEN: usize = 12;
pub const BLOCK_BYTES: usize = 512;
/// The data buffer's 128 blocks (card::MAX_BLOCKS), the most one request
/// moves.
pub const MAX_RW_BLOCKS: u32 = 128;
pub const CAPACITY_LEN: usize = 8;
