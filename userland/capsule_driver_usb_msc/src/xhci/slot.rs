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

//! Slots and addresses on driver.xhci0.

use super::call::call;
use super::wire::{DATA_AT, OP_ADDRESS_DEVICE, OP_DISABLE_SLOT, OP_ENABLE_SLOT};
use crate::protocol::E_IO;

pub fn enable_slot(xhci: u32) -> Result<u8, i32> {
    let mut resp = [0u8; DATA_AT + 4];
    let len = call(xhci, OP_ENABLE_SLOT, &[], &mut resp)?;
    match resp[DATA_AT] {
        0 => Err(E_IO),
        slot if len >= 4 => Ok(slot),
        _ => Err(E_IO),
    }
}

pub fn disable_slot(xhci: u32, slot: u8) {
    let mut resp = [0u8; DATA_AT];
    /*
     * A slot that cannot be disabled stays allocated in the controller
     * driver's table; there is nothing further this driver can do.
     */
    let _ = call(xhci, OP_DISABLE_SLOT, &[slot], &mut resp);
}

/// Reset the port and address its device on `slot`; `E_BUSY` when another
/// class driver holds the port.
pub fn address_device(xhci: u32, slot: u8, port: u8) -> Result<(), i32> {
    let mut resp = [0u8; DATA_AT + 8];
    let len = call(xhci, OP_ADDRESS_DEVICE, &[slot, port], &mut resp)?;
    if len < 8 || resp[DATA_AT] != slot || resp[DATA_AT + 1] != port {
        return Err(E_IO);
    }
    Ok(())
}
