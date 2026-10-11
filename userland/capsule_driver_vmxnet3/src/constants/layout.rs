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

//! The control grant the device and the driver share: the DriverShared
//! area, the queue descriptor table, and every ring. Rings must start on
//! 512 bytes and the queue table on 128 (vmxnet3_defs.h), and these offsets
//! keep both.

use super::ring::{DESC_BYTES, RX_COMP, RX_RING, TX_COMP, TX_DATA_DESC, TX_RING};

pub const SHARED_OFF: usize = 0x000;
pub const TXQ_DESC_OFF: usize = 0x800;
pub const RXQ_DESC_OFF: usize = 0x900;
/// One transmit and one receive queue descriptor, 256 bytes each.
pub const QUEUE_DESC_LEN: usize = 0x200;
pub const TX_RING_OFF: usize = 0xA00;
pub const TX_COMP_OFF: usize = 0xC00;
pub const RX_RING_OFF: usize = 0xE00;
pub const RX_RING1_OFF: usize = 0x1000;
pub const RX_COMP_OFF: usize = 0x1200;
pub const TX_DATA_OFF: usize = 0x2000;
pub const CONTROL_BYTES: usize = 0x3000;

const _: () = assert!(TXQ_DESC_OFF & 127 == 0 && RXQ_DESC_OFF == TXQ_DESC_OFF + 256);
const _: () = assert!(TX_RING_OFF == TXQ_DESC_OFF + QUEUE_DESC_LEN);
const _: () = assert!(TX_RING_OFF + TX_RING * DESC_BYTES <= TX_COMP_OFF);
const _: () = assert!(TX_COMP_OFF + TX_COMP * DESC_BYTES <= RX_RING_OFF);
const _: () = assert!(RX_RING_OFF + RX_RING * DESC_BYTES <= RX_RING1_OFF);
const _: () = assert!(RX_RING1_OFF + RX_RING * DESC_BYTES <= RX_COMP_OFF);
const _: () = assert!(RX_COMP_OFF + RX_COMP * DESC_BYTES <= TX_DATA_OFF);
const _: () = assert!(TX_DATA_OFF + TX_RING * TX_DATA_DESC <= CONTROL_BYTES);
const _: () = assert!((TX_RING_OFF | TX_COMP_OFF | RX_RING_OFF | RX_RING1_OFF) & 511 == 0);
const _: () = assert!((RX_COMP_OFF | TX_DATA_OFF) & 511 == 0);
