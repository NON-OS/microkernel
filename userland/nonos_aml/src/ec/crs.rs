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

//! The EC's ports from its _CRS (ACPI 6.5, 12.11): the first I/O resource
//! is the data port, the second the command and status port. Both the I/O
//! port descriptor (6.4.2.5) and the fixed I/O descriptor (6.4.2.6) count.

use super::regs::EcPorts;
use alloc::vec::Vec;

const SMALL_IO: u8 = 0x08;
const SMALL_FIXED_IO: u8 = 0x09;
const SMALL_END: u8 = 0x0F;

/// The base port of each I/O resource in the template, in order.
fn io_bases(t: &[u8]) -> Vec<u16> {
    let (mut out, mut i) = (Vec::new(), 0usize);
    while let Some(&tag) = t.get(i) {
        if tag & 0x80 != 0 {
            let Some(l) = t.get(i + 1..i + 3) else { break };
            i += 3 + u16::from_le_bytes([l[0], l[1]]) as usize;
            continue;
        }
        let (kind, len) = ((tag >> 3) & 0x0F, (tag & 0x07) as usize);
        let Some(body) = t.get(i + 1..i + 1 + len) else { break };
        match kind {
            SMALL_END => break,
            SMALL_IO if len >= 7 => out.push(u16::from_le_bytes([body[1], body[2]])),
            SMALL_FIXED_IO if len >= 3 => out.push(u16::from_le_bytes([body[0], body[1]]) & 0x3FF),
            _ => {}
        }
        i += 1 + len;
    }
    out
}

pub fn ec_ports(crs: &[u8]) -> Option<EcPorts> {
    let bases = io_bases(crs);
    Some(EcPorts { data: *bases.first()?, command: *bases.get(1)? })
}
