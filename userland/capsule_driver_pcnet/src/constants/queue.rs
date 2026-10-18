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

pub const DESC_BYTES: usize = 16;

/// Ring lengths reach the part as log2 in the init block, so they are
/// powers of two, 512 at most.
pub const RX_LOG2: u8 = 5;
pub const TX_LOG2: u8 = 5;
pub const RX_DESC_COUNT: usize = 1 << RX_LOG2;
pub const TX_DESC_COUNT: usize = 1 << TX_LOG2;

/// A full frame with its FCS fits, and a multiple of 16 keeps every slot
/// aligned in the pool.
pub const BUFFER_LEN: usize = 1536;
pub const RX_POOL_BYTES: usize = RX_DESC_COUNT * BUFFER_LEN;
pub const TX_POOL_BYTES: usize = TX_DESC_COUNT * BUFFER_LEN;

// Descriptor status: the high half of RMD1 and TMD1.
pub const OWN: u16 = 1 << 15;
pub const ERR: u16 = 1 << 14;
pub const STP: u16 = 1 << 9;
pub const ENP: u16 = 1 << 8;
/// The top four bits of a byte count must read as ones, or the part treats
/// the descriptor as corrupt.
pub const BCNT_ONES: u16 = 0xF000;
/// The received length, FCS included: the low twelve bits of RMD2.
pub const MCNT_MASK: u32 = 0x0FFF;

/// A buffer's length as the part takes it: two's complement, ones on top.
pub const fn bcnt(len: usize) -> u16 {
    (0u16.wrapping_sub(len as u16) & 0x0FFF) | BCNT_ONES
}
