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

//! Ring and buffer sizes. Both advanced descriptor formats are 16 bytes
//! (union igc_adv_rx_desc, union igc_adv_tx_desc). RDLEN and TDLEN must be
//! a multiple of 128 bytes; 32 descriptors are 512.

pub const DESC_BYTES: usize = 16;

pub const RX_DESC_COUNT: usize = 32;
pub const TX_DESC_COUNT: usize = 32;

pub const RX_RING_BYTES: usize = RX_DESC_COUNT * DESC_BYTES;
pub const TX_RING_BYTES: usize = TX_DESC_COUNT * DESC_BYTES;

/// IGC_RXBUFFER_2048, the size SRRCTL.BSIZEPKT is programmed with.
pub const RX_BUFFER_LEN: usize = 2048;
pub const TX_BUFFER_LEN: usize = 2048;

pub const RX_BUFFER_POOL_BYTES: usize = RX_DESC_COUNT * RX_BUFFER_LEN;
pub const TX_BUFFER_POOL_BYTES: usize = TX_DESC_COUNT * TX_BUFFER_LEN;

const _: () = assert!(RX_RING_BYTES.is_multiple_of(128) && TX_RING_BYTES.is_multiple_of(128));
