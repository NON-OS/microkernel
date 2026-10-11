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

pub const RX_DESC_COUNT: usize = 32;
pub const TX_DESC_COUNT: usize = 32;

/// A full frame with its FCS fits, and a multiple of 16 keeps every slot
/// aligned in the pool. Below the 2047 a descriptor's size field holds.
pub const BUFFER_LEN: usize = 1536;
pub const RX_POOL_BYTES: usize = RX_DESC_COUNT * BUFFER_LEN;
pub const TX_POOL_BYTES: usize = TX_DESC_COUNT * BUFFER_LEN;

/// Descriptor word 0: the part owns it while set.
pub const OWN: u32 = 1 << 31;
/// Receive status: frame length (FCS included), error summary, first and
/// last descriptor of the frame.
pub const RDES0_FL_SHIFT: u32 = 16;
pub const RDES0_FL_MASK: u32 = 0x3FFF;
pub const RDES0_ES: u32 = 1 << 15;
pub const RDES0_FS: u32 = 1 << 9;
pub const RDES0_LS: u32 = 1 << 8;
/// Receive control: end of ring.
pub const RDES1_RER: u32 = 1 << 25;
/// Transmit control: first and last segment, a setup frame, end of ring.
pub const TDES1_LS: u32 = 1 << 30;
pub const TDES1_FS: u32 = 1 << 29;
pub const TDES1_SET: u32 = 1 << 27;
pub const TDES1_TER: u32 = 1 << 25;
/// Buffer 1's size, the low eleven bits of word 1.
pub const SIZE_MASK: u32 = 0x7FF;

/// The setup frame for perfect filtering: sixteen addresses, each as three
/// sixteen-bit halves in the low half of a longword.
pub const SETUP_FRAME_LEN: usize = 192;
