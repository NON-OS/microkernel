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

//! The interrupts a resource template names (ACPI 6.5, 6.4): the small IRQ
//! descriptor (6.4.2.1) and the large Extended Interrupt descriptor
//! (6.4.3.6), in order. Every other descriptor is stepped over; a template
//! that runs past its buffer stops there.

use alloc::vec::Vec;

const SMALL_IRQ: u8 = 0x04;
const SMALL_END: u8 = 0x0F;
const LARGE_EXTENDED_IRQ: u8 = 0x09;

/// Each interrupt in the template, one per descriptor that names any: the
/// lowest set bit of an IRQ descriptor's mask, or an Extended Interrupt
/// descriptor's first number.
pub fn interrupts(t: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(&tag) = t.get(i) {
        if tag & 0x80 == 0 {
            let (kind, len) = ((tag >> 3) & 0x0F, (tag & 0x07) as usize);
            if kind == SMALL_END {
                break;
            }
            let Some(body) = t.get(i + 1..i + 1 + len) else { break };
            if kind == SMALL_IRQ && len >= 2 {
                let mask = u16::from_le_bytes([body[0], body[1]]);
                if mask != 0 {
                    out.push(mask.trailing_zeros());
                }
            }
            i += 1 + len;
        } else {
            let Some(l) = t.get(i + 1..i + 3) else { break };
            let len = u16::from_le_bytes([l[0], l[1]]) as usize;
            let Some(body) = t.get(i + 3..i + 3 + len) else { break };
            if tag & 0x7F == LARGE_EXTENDED_IRQ && len >= 6 && body[1] >= 1 {
                out.push(u32::from_le_bytes([body[2], body[3], body[4], body[5]]));
            }
            i += 3 + len;
        }
    }
    out
}
