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

//! Buffer descriptor and status block fields (tg3.h, struct
//! tg3_rx_buffer_desc, tg3_tx_buffer_desc and tg3_hw_status). With
//! GRC_MODE_WSWAP_NONFRM_DATA set on a little-endian host, each 32-bit field
//! reads and writes natively at its offset, as Linux uses them.

pub const RX_BD_BYTES: usize = 32;
pub const TX_BD_BYTES: usize = 16;

pub const RXD_LEN_MASK: u32 = 0x0000_FFFF;
pub const RXD_FLAG_END: u32 = 0x0004;
pub const RXD_ERR_BAD_CRC: u32 = 0x0001_0000;
pub const RXD_ERR_COLLISION: u32 = 0x0002_0000;
pub const RXD_ERR_LINK_LOST: u32 = 0x0004_0000;
pub const RXD_ERR_PHY_DECODE: u32 = 0x0008_0000;
pub const RXD_ERR_MAC_ABRT: u32 = 0x0020_0000;
pub const RXD_ERR_TOO_SMALL: u32 = 0x0040_0000;
pub const RXD_ERR_NO_RESOURCES: u32 = 0x0080_0000;
pub const RXD_ERR_HUGE_FRAME: u32 = 0x0100_0000;
/// RXD_ERR_MASK: every error but the odd nibble on MII.
pub const RXD_ERR_MASK: u32 = RXD_ERR_BAD_CRC
    | RXD_ERR_COLLISION
    | RXD_ERR_LINK_LOST
    | RXD_ERR_PHY_DECODE
    | RXD_ERR_MAC_ABRT
    | RXD_ERR_TOO_SMALL
    | RXD_ERR_NO_RESOURCES
    | RXD_ERR_HUGE_FRAME;
pub const RXD_OPAQUE_INDEX_MASK: u32 = 0x0000_FFFF;
pub const RXD_OPAQUE_RING_STD: u32 = 0x0001_0000;
pub const RXD_OPAQUE_RING_MASK: u32 = 0x0007_0000;

pub const TXD_FLAG_END: u32 = 0x0004;
pub const TXD_LEN_SHIFT: u32 = 16;

/// Status block: idx[0].rx_producer and .tx_consumer, little-endian u16s.
pub const STATUS_RX_PRODUCER: usize = 16;
pub const STATUS_TX_CONSUMER: usize = 18;
