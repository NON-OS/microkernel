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

//! How this driver lays its three DMA grants out. The ring sizes are the
//! part's (constants/ring.rs); how many buffers back them is this driver's
//! choice, as tg3's rx_pending (200 of 512) is Linux's.

use super::desc::{RX_BD_BYTES, TX_BD_BYTES};
use super::ring::{HW_STATUS_SIZE, RX_RET_RING_SIZE, RX_STD_RING_SIZE, TX_RING_SIZE};

const PAGE: usize = 4096;

/// One grant for the rings and the status block, each on its own pages.
pub const STD_RING_AT: usize = 0;
pub const RET_RING_AT: usize = STD_RING_AT + RX_STD_RING_SIZE * RX_BD_BYTES;
pub const TX_RING_AT: usize = RET_RING_AT + RX_RET_RING_SIZE * RX_BD_BYTES;
pub const STATUS_AT: usize = TX_RING_AT + TX_RING_SIZE * TX_BD_BYTES;
pub const RINGS_BYTES: usize = STATUS_AT + PAGE;

/// Receive buffers the part holds at once, each a 2 KiB slot: room for the
/// 1536 bytes a standard BD declares and the 64-byte DMA slack tg3 maps
/// beside it (TG3_DMA_BYTE_ENAB).
pub const RX_BUFFERS: usize = 64;
pub const RX_BUFFER_LEN: usize = 2048;
pub const RX_BUFFER_POOL_BYTES: usize = RX_BUFFERS * RX_BUFFER_LEN;

/// Transmit slots: at most this many frames are in flight, each in its own
/// 2 KiB slot, though the ring has TX_RING_SIZE entries.
pub const TX_BUFFERS: usize = 64;
pub const TX_BUFFER_LEN: usize = 2048;
pub const TX_BUFFER_POOL_BYTES: usize = TX_BUFFERS * TX_BUFFER_LEN;

const _: () = assert!(RET_RING_AT.is_multiple_of(PAGE) && TX_RING_AT.is_multiple_of(PAGE));
const _: () = assert!(STATUS_AT.is_multiple_of(PAGE) && HW_STATUS_SIZE <= PAGE);
// The windows of posted buffers and in-flight frames repeat around the rings.
const _: () = assert!(RX_STD_RING_SIZE.is_multiple_of(RX_BUFFERS));
const _: () = assert!(TX_RING_SIZE.is_multiple_of(TX_BUFFERS));
