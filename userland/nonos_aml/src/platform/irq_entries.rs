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

//! Every interrupt a template names and how it signals: one per mask bit
//! of an IRQ descriptor (ACPI 6.5, 6.4.2.1), one per number of an Extended
//! Interrupt descriptor (6.4.3.6). `interrupts` keeps one.

use alloc::vec::Vec;

const SMALL_IRQ: u8 = 0x04;
const SMALL_END: u8 = 0x0F;
const LARGE_EXTENDED_IRQ: u8 = 0x09;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interrupt {
    pub gsiv: u32,
    pub edge: bool,
    pub active_low: bool,
    pub shared: bool,
    pub wake: bool,
}

fn entry(gsiv: u32, f: u8, bits: [u8; 4]) -> Interrupt {
    let [edge, active_low, shared, wake] = bits.map(|b| f & b != 0);
    Interrupt { gsiv, edge, active_low, shared, wake }
}

/// A small IRQ descriptor without its third byte is edge, active high.
fn small(body: &[u8], out: &mut Vec<Interrupt>) {
    let mask = u16::from_le_bytes([body[0], body[1]]);
    let f = body.get(2).copied().unwrap_or(1);
    out.extend((0..16).filter(|b| mask & (1 << b) != 0).map(|n| entry(n, f, [1, 8, 16, 32])));
}

/// A count its length cannot hold is a lie: none is taken, not the few.
fn extended(body: &[u8], out: &mut Vec<Interrupt>) {
    let (f, n) = (body[0], body[1] as usize);
    let Some(numbers) = body.get(2..2 + 4 * n) else { return };
    let gsiv = |c: &[u8]| u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
    out.extend(numbers.chunks_exact(4).map(|c| entry(gsiv(c), f, [2, 4, 8, 16])));
}

pub fn interrupt_entries(t: &[u8]) -> Vec<Interrupt> {
    let (mut out, mut i) = (Vec::new(), 0usize);
    while let Some(&tag) = t.get(i) {
        let (kind, head, len) = if tag & 0x80 == 0 {
            ((tag >> 3) & 0x0F, 1, (tag & 0x07) as usize)
        } else {
            let Some(l) = t.get(i + 1..i + 3) else { break };
            (tag & 0x7F | 0x80, 3, u16::from_le_bytes([l[0], l[1]]) as usize)
        };
        let Some(body) = t.get(i + head..i + head + len) else { break };
        match kind {
            SMALL_END => break,
            SMALL_IRQ if (2..=3).contains(&len) => small(body, &mut out),
            k if k == LARGE_EXTENDED_IRQ | 0x80 && len >= 2 => extended(body, &mut out),
            _ => {}
        }
        i += head + len;
    }
    out
}
