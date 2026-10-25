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

//! A root port's speed in driver.xhci0's OP_PORT_STATUS data: a count, three
//! reserved bytes, then 8 bytes a port (its number, then its PORTSC at 4).
//! Pure, for the proofs.

const ENTRY: usize = 8;

/// Current Connect Status, PORTSC bit 0 (xHCI 1.2 5.4.8 as recalled, unverified). CSC is
/// left alone: driver.xhci0 clears it on read, for every class driver.
const PORTSC_CCS: u32 = 1;

fn portsc_of(data: &[u8], port: u8) -> Option<u32> {
    let count = *data.first()? as usize;
    let entries = data.get(4..4 + count * ENTRY)?;
    let e = entries.chunks_exact(ENTRY).find(|e| e[0] == port)?;
    Some(u32::from_le_bytes([e[4], e[5], e[6], e[7]]))
}

/// The Port Speed id of `port` (PORTSC bits 13:10, xHCI 1.2 5.4.8 as
/// recalled, unverified).
pub fn speed_in(data: &[u8], port: u8) -> Option<u8> {
    Some(((portsc_of(data, port)? >> 10) & 0xF) as u8)
}

/// Whether a device is on `port`: CCS set. A port the data does not name
/// has nothing on it.
pub fn connected_in(data: &[u8], port: u8) -> bool {
    portsc_of(data, port).is_some_and(|sc| sc & PORTSC_CCS != 0)
}
