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

//! IVHD special entries (48h, spec 48882 5.2.2.2): each IOAPIC and HPET by
//! its handle, the IOAPIC id the MADT gives, and the requester id its
//! interrupts arrive under, which AMD interrupt remapping keys its table
//! by. Pure; a 10h and an 11h block naming the same one count once.

use super::ivhd_entry::size;

pub const IOAPIC: u8 = 1;
pub const HPET: u8 = 2;
pub const MAX_SPECIALS: usize = 16;
const FIRST_BLOCK: usize = 48;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Special {
    pub variety: u8,
    pub handle: u8,
    pub id: u16,
}

/// Special entries in table order, written to `out`; returns how many.
pub fn specials(table: &[u8], out: &mut [Special]) -> usize {
    let (mut n, mut at) = (0, FIRST_BLOCK);
    while at + 4 <= table.len() {
        let length = u16::from_le_bytes([table[at + 2], table[at + 3]]) as usize;
        if length < 4 || at + length > table.len() {
            break;
        }
        let header = match table[at] {
            0x10 => 24,
            0x11 | 0x40 => 40,
            _ => length,
        };
        if header < length {
            n = block(&table[at + header..at + length], out, n);
        }
        at += length;
    }
    n
}

fn block(bytes: &[u8], out: &mut [Special], mut n: usize) -> usize {
    let mut at = 0;
    while at + 4 <= bytes.len() {
        let Some(len) = size(bytes, at).filter(|len| at + len <= bytes.len()) else {
            break;
        };
        if bytes[at] == 0x48 && len == 8 {
            let id = u16::from_le_bytes([bytes[at + 5], bytes[at + 6]]);
            let s = Special { variety: bytes[at + 7], handle: bytes[at + 4], id };
            if !out[..n].contains(&s) && n < out.len() {
                out[n] = s;
                n += 1;
            }
        }
        at += len;
    }
    n
}
