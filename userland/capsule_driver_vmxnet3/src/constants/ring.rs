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

//! Ring sizes and the descriptor bits this driver uses, per vmxnet3_defs.h.

pub const DESC_BYTES: usize = 16;
/// Ring sizes must be multiples of 32.
pub const TX_RING: usize = 32;
pub const RX_RING: usize = 32;
/// Receive ring 1 takes body buffers for frames larger than a head buffer.
/// None is ever larger here, so ring 1 is given to the device empty.
pub const TX_COMP: usize = TX_RING;
pub const RX_COMP: usize = 2 * RX_RING;
/// The transmit data ring is part of the revision 1 queue and must exist,
/// though this driver copies whole frames into its own pool instead.
pub const TX_DATA_DESC: usize = 128;
pub const BUFFER_LEN: usize = 2048;
pub const RX_POOL_BYTES: usize = RX_RING * BUFFER_LEN;
pub const TX_POOL_BYTES: usize = TX_RING * BUFFER_LEN;

/// The generation every ring starts at; a descriptor is the device's while
/// its gen bit equals the ring's.
pub const INIT_GEN: u32 = 1;
pub const LEN_MASK: u32 = 0x3FFF;
// Transmit descriptor, dwords 2 and 3.
pub const TXD_GEN_SHIFT: u32 = 14;
pub const TXD_EOP: u32 = 1 << 12;
pub const TXD_CQ: u32 = 1 << 13;
// Receive descriptor, dword 2; buffer type 0 is a head buffer.
pub const RXD_GEN_SHIFT: u32 = 31;
// Completion descriptors: gen in bit 31 of dword 3, the index in dword 0.
pub const COMP_GEN_SHIFT: u32 = 31;
pub const COMP_IDX_MASK: u32 = 0xFFF;
pub const RCD_EOP: u32 = 1 << 14;
pub const RCD_SOP: u32 = 1 << 15;
pub const RCD_RQID_SHIFT: u32 = 16;
pub const RCD_RQID_MASK: u32 = 0x3FF;
pub const RCD_ERR: u32 = 1 << 14;
