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

//! Event TRBs as the controller lays them out (xHCI 1.2 section 6.4.2).
//!
//! Built field by field from the specification rather than through the
//! driver's getters, so a getter that read the wrong bits would not agree
//! with these by construction.

use crate::constants::{TRB_TYPE_CMD_COMPLETION_EVENT, TRB_TYPE_TRANSFER_EVENT};
use crate::trb::Trb;

pub const PORT_STATUS_CHANGE: u32 = 34;
pub const CC_SUCCESS: u8 = 1;
pub const CC_TRB_ERROR: u8 = 5;
pub const CC_STALL: u8 = 6;
pub const CC_SHORT_PACKET: u8 = 13;

fn event(ty: u32, pointer: u64, d2: u32, d3_high: u32) -> Trb {
    Trb { d0: pointer as u32, d1: (pointer >> 32) as u32, d2, d3: d3_high | ((ty & 0x3F) << 10) }
}

/// A Transfer Event: TRB pointer, 24-bit residual length, completion code,
/// endpoint id in bits 20:16 and slot id in bits 31:24 of the last dword.
pub fn transfer(pointer: u64, code: u8, residual: u32, slot: u8, endpoint: u8) -> Trb {
    let d2 = ((code as u32) << 24) | (residual & 0x00FF_FFFF);
    let d3 = ((slot as u32) << 24) | (((endpoint as u32) & 0x1F) << 16);
    event(TRB_TYPE_TRANSFER_EVENT, pointer, d2, d3)
}

/// A Command Completion Event: command TRB pointer, completion code, slot id.
pub fn command(pointer: u64, code: u8, slot: u8) -> Trb {
    event(TRB_TYPE_CMD_COMPLETION_EVENT, pointer, (code as u32) << 24, (slot as u32) << 24)
}

/// A Port Status Change Event for `port`, which nobody waits on.
pub fn port_change(port: u8) -> Trb {
    event(PORT_STATUS_CHANGE, (port as u64) << 24, (CC_SUCCESS as u32) << 24, 0)
}

/// A Port Status Change Event carrying `n` in the dword the specification
/// reserves, so a test can tell which write the driver read.
pub fn numbered(n: u32) -> Trb {
    let mut t = port_change(1);
    t.d1 = n;
    t
}
