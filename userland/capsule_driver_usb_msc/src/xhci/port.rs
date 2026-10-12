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
use super::wire::{DATA_AT, OP_PORT_STATUS};
use crate::protocol::E_IO;

/// A root port that has a device on it, and which class driver holds it.
#[derive(Clone, Copy)]
pub struct Port {
    pub id: u8,
    pub owner: u8,
}

const PORTSC_CONNECTED: u32 = 1;

/// The Port Speed id of `port` (PORTSC bits 13:10): 3 high speed, 4 and
/// up SuperSpeed; `None` when the host did not say.
pub fn port_speed(xhci: u32, port: u8) -> Option<u8> {
    let mut resp = [0u8; DATA_AT + 4 + 255 * 8];
    let len = call(xhci, OP_PORT_STATUS, &[], &mut resp).ok()?;
    let count = (resp[DATA_AT] as usize).min(len.saturating_sub(4) / 8);
    let entries = resp[DATA_AT + 4..DATA_AT + 4 + count * 8].chunks_exact(8);
    let e = entries.into_iter().find(|e| e[0] == port)?;
    Some(((u32::from_le_bytes([e[4], e[5], e[6], e[7]]) >> 10) & 0xF) as u8)
}

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
        .filter(|e| u32::from_le_bytes([e[4], e[5], e[6], e[7]]) & PORTSC_CONNECTED != 0)
        .map(|e| Port { id: e[0], owner: e[1] })
        .collect())
}
