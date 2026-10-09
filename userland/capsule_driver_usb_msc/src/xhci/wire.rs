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

//! The driver.xhci0 wire: a 20-byte header, then for replies a status word.
//! Mirrors `capsule_driver_xhci/src/protocol/{header,ops,limits}.rs`.

pub const MAGIC: u32 = 0x4E58_4843;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
/// Where a reply's data starts: after the header and the status word.
pub const DATA_AT: usize = HDR_LEN + 4;
/// The most one bulk transfer moves: the controller driver's DMA page.
pub const BULK_MAX: usize = 4096;

pub const OP_PORT_STATUS: u16 = 0x0003;
pub const OP_ENABLE_SLOT: u16 = 0x0004;
pub const OP_DISABLE_SLOT: u16 = 0x0005;
pub const OP_ADDRESS_DEVICE: u16 = 0x0006;
pub const OP_GET_CONFIG_DESCRIPTOR: u16 = 0x0008;
pub const OP_CONTROL_TRANSFER: u16 = 0x000B;
pub const OP_CONFIGURE_BULK: u16 = 0x0010;
pub const OP_BULK_OUT: u16 = 0x0011;
pub const OP_BULK_IN: u16 = 0x0012;
pub const OP_RESET_BULK: u16 = 0x0013;

/// A port's `owner` byte in `OP_PORT_STATUS`: free, being classified by a
/// class driver, or claimed by one.
pub const PORT_FREE: u8 = 0;
pub const PORT_CLAIMED: u8 = 2;
/// The xHCI status for a busy port and for an endpoint that stalled.
pub const E_BUSY: i32 = -16;
pub const E_PIPE: i32 = -32;
