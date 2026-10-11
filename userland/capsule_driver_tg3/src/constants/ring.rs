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

//! Ring control blocks and ring sizes for a part that is JUMBO_CAPABLE but
//! not LRG_PROD_RING_CAP, which is the 57765 class (tg3.h, tg3.c).

pub const TG3_BDINFO_SIZE: u32 = 0x10;
pub const TG3_BDINFO_HOST_ADDR: u32 = 0x0;
pub const TG3_BDINFO_MAXLEN_FLAGS: u32 = 0x8;
pub const TG3_BDINFO_NIC_ADDR: u32 = 0xC;
pub const BDINFO_FLAGS_DISABLED: u32 = 0x0000_0002;
pub const BDINFO_FLAGS_MAXLEN_SHIFT: u32 = 16;
pub const RCVDBDI_JUMBO_BD: usize = 0x2440;
pub const RCVDBDI_STD_BD: usize = 0x2450;

/// TG3_RX_STD_MAX_SIZE_5700: the standard producer ring's entries.
pub const RX_STD_RING_SIZE: usize = 512;
/// TG3_RX_RET_MAX_SIZE_5700, tg3_rx_ret_ring_size for JUMBO_CAPABLE.
pub const RX_RET_RING_SIZE: usize = 1024;
/// TG3_TX_RING_SIZE.
pub const TX_RING_SIZE: usize = 512;
/// TG3_RX_STD_DMA_SZ, the buffer size each standard BD declares.
pub const RX_STD_DMA_SZ: u32 = 1536;
/// TG3_HW_STATUS_SIZE.
pub const HW_STATUS_SIZE: usize = 0x50;

/// What tg3_tx_rcbs_disable and tg3_rx_ret_rcbs_disable leave enabled for
/// the 57765 class: the first send ring of two, the first return ring of four.
pub const SEND_RCBS: u32 = 2;
pub const RET_RCBS: u32 = 4;
