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

//! The bulk pipes driver.xhci0 keeps for a slot: added once at bind, and
//! brought back on the controller side after a stall.

use super::call::call;
use super::wire::{DATA_AT, OP_CONFIGURE_BULK, OP_RESET_BULK};
use crate::bus::Pipes;

/// Add the bulk pipes to the slot. Byte 3 carries each pipe's SuperSpeed
/// bMaxBurst, IN in bits 3:0 and OUT in bits 7:4.
pub fn configure_bulk(xhci: u32, slot: u8, p: &Pipes) -> Result<(), i32> {
    let bursts = (p.burst_in & 0x0F) | (p.burst_out & 0x0F) << 4;
    let mut body = [slot, p.bulk_in, p.bulk_out, bursts, 0, 0, 0, 0];
    body[4..6].copy_from_slice(&p.max_packet_in.to_le_bytes());
    body[6..8].copy_from_slice(&p.max_packet_out.to_le_bytes());
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_CONFIGURE_BULK, &body, &mut resp).map(|_| ())
}

/// Bring a stalled pipe back on the controller side.
pub fn reset_bulk(xhci: u32, slot: u8, dir_in: bool) -> Result<(), i32> {
    let mut resp = [0u8; DATA_AT + 4];
    call(xhci, OP_RESET_BULK, &[slot, dir_in as u8], &mut resp).map(|_| ())
}
