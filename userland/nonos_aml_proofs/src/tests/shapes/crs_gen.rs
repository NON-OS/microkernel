// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded resource templates (ACPI 6.5, 6.4): IRQ, Extended Interrupt
//! and other descriptors, an end tag, and at most one lie: a length the
//! descriptor does not have, a count its length cannot hold, a cut, no end
//! tag, or bytes after it. When the lie leaves it known, the template
//! comes with what an honest reader finds, written as it was built.

use super::gen::Rng;

#[derive(Default)]
pub struct Truth {
    pub items: usize,
    /// One per descriptor that names any: its lowest IRQ or first number.
    pub first: Vec<u32>,
    pub all: Vec<u32>,
}

pub(super) fn descriptor(r: &mut Rng, t: &mut Vec<u8>, truth: &mut Truth, ext: &mut Vec<usize>) {
    match r.below(4) {
        0 => {
            let mask = (r.next() & r.next() & r.next()) as u16;
            let bits: Vec<u32> = (0..16).filter(|b| mask & 1 << b != 0).collect();
            truth.first.extend(bits.first());
            truth.all.extend(&bits);
            t.extend([0x22, mask as u8, (mask >> 8) as u8]);
            if r.below(2) == 0 {
                let at = t.len() - 3;
                t[at] = 0x23;
                t.push(r.next() as u8 & 0x39);
            }
        }
        1 => {
            let n = 1 + r.below(3) as usize;
            let numbers: Vec<u32> = (0..n).map(|_| r.below(1024) as u32).collect();
            ext.push(t.len());
            t.extend([0x89, 2 + 4 * n as u8, 0, r.next() as u8 & 0x1F, n as u8]);
            numbers.iter().for_each(|g| t.extend(g.to_le_bytes()));
            truth.first.push(numbers[0]);
            truth.all.extend(numbers);
        }
        2 => t.extend([0x47, 1, 0x60, 0, 0x60, 0, 1, 1]),
        _ => t.extend([0x86, 9, 0, 1, 0, 0, 0xD0, 0xFE, 0, 0x10, 0, 0]),
    }
    truth.items += 1;
}

#[path = "crs_lie.rs"]
mod lie;
pub use lie::template;
