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

//! The root ports driver.xhci0 sees a device on.

use alloc::vec::Vec;

use super::call::call;
use super::wire::{DATA_AT, E_IO, OP_PORT_STATUS};
use crate::scan::Port;

/// Current Connect Status and Connect Status Change (xHCI 1.2, 5.4.8).
const PORTSC_CCS: u32 = 1;
const PORTSC_CSC: u32 = 1 << 17;

/// Every root port with a device connected.
pub fn connected_ports(xhci: u32) -> Result<Vec<Port>, i32> {
    let mut resp = [0u8; DATA_AT + 4 + 255 * 8];
    let len = call(xhci, OP_PORT_STATUS, &[], &mut resp)?;
    let count = resp[DATA_AT] as usize;
    if len < 4 + count * 8 {
        return Err(E_IO);
    }
    let entries = resp[DATA_AT + 4..DATA_AT + 4 + count * 8].chunks_exact(8);
    Ok(entries
        .map(|e| (e[0], e[1], u32::from_le_bytes([e[4], e[5], e[6], e[7]])))
        .filter(|&(_, _, sc)| sc & PORTSC_CCS != 0)
        .map(|(id, owner, sc)| Port { id, owner, changed: sc & PORTSC_CSC != 0 })
        .collect())
}
