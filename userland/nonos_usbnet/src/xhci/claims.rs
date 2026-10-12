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
//! A device shared between class drivers: the interfaces this driver
//! drives claimed, and the shared slot on a port asked for, so a second
//! driver joins the device the first configured instead of addressing it
//! again.

use super::call::call;
use super::wire::{DATA_AT, OP_CLAIM_INTERFACE, OP_SLOT_OF_PORT};

/// 0, or -16 when another driver holds `interface`.
pub fn claim_interface(xhci: u32, slot: u8, port: u8, interface: u8) -> Result<(), i32> {
    let mut resp = [0u8; DATA_AT];
    call(xhci, OP_CLAIM_INTERFACE, &[slot, port, interface], &mut resp).map(|_| ())
}

/// The shared slot on `port` and its claimed interfaces, if one is there.
pub fn slot_of_port(xhci: u32, port: u8) -> Option<(u8, u32)> {
    let mut resp = [0u8; DATA_AT + 8];
    if call(xhci, OP_SLOT_OF_PORT, &[port], &mut resp).ok()? < 8 {
        return None;
    }
    let held = u32::from_le_bytes([
        resp[DATA_AT + 4],
        resp[DATA_AT + 5],
        resp[DATA_AT + 6],
        resp[DATA_AT + 7],
    ]);
    Some((resp[DATA_AT], held))
}
