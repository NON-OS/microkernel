// NONOS Operating System (AGPL-3.0-or-later)
//! _CRS templates under lying shapes. The model (ACPI 6.5, 6.4; recalled,
//! unverified): a small descriptor is a tag byte, name in bits 6:3 and a
//! length in bits 2:0; a large one a tag with bit 7 set and a 16-bit
//! length. The walk stops at the end tag (small name 0xF) or at the first
//! descriptor whose length runs past the buffer, keeping what came before.
//! An IRQ descriptor is 2 or 3 bytes long, else it is stepped over; an
//! Extended Interrupt descriptor whose count its length cannot hold names
//! nothing. Honest templates are also held to what was built.

use super::crs_gen::template;
use super::gen::{Rng, FLOOR, SEEDS};
use crate::platform::{interrupt_entries, interrupts, items, Interrupt};

/// (large, name, whole descriptor, body), as the model walks them.
fn frames(t: &[u8]) -> Vec<(bool, u8, &[u8], &[u8])> {
    let (mut out, mut i) = (Vec::new(), 0);
    while i < t.len() {
        let large = t[i] >= 0x80;
        let (name, head, len) = match large {
            true if i + 3 > t.len() => break,
            true => (t[i] - 0x80, 3, usize::from(t[i + 1]) | usize::from(t[i + 2]) << 8),
            false => (t[i] / 8, 1, usize::from(t[i] % 8)),
        };
        if (!large && name == 0xF) || i + head + len > t.len() {
            break;
        }
        out.push((large, name, &t[i..i + head + len], &t[i + head..i + head + len]));
        i += head + len;
    }
    out
}

fn entries(t: &[u8]) -> Vec<Interrupt> {
    let mut out = Vec::new();
    let flag = |f: u8, b: [u8; 4], gsiv: u32| Interrupt {
        gsiv,
        edge: f & b[0] != 0,
        active_low: f & b[1] != 0,
        shared: f & b[2] != 0,
        wake: f & b[3] != 0,
    };
    for (large, name, _, b) in frames(t) {
        if !large && name == 4 && (b.len() == 2 || b.len() == 3) {
            let mask = u16::from(b[0]) | u16::from(b[1]) << 8;
            let f = if b.len() == 3 { b[2] } else { 1 };
            out.extend((0..16).filter(|n| mask >> n & 1 == 1).map(|n| flag(f, [1, 8, 16, 32], n)));
        } else if large && name == 9 && b.len() >= 2 && b.len() >= 2 + 4 * usize::from(b[1]) {
            let n = b[2..2 + 4 * usize::from(b[1])].chunks(4);
            let g = n.map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]));
            out.extend(g.map(|g| flag(b[0], [2, 4, 8, 16], g)));
        }
    }
    out
}

#[path = "crs_hold.rs"]
mod hold;
