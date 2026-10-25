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

//! CTA-861 extensions whose data block lengths, tags and descriptor
//! offset may each be honest or a lie.

use super::gen::{slot, Kind};
use super::rng::Rng;
use crate::build::seal;

/// The block, and the kinds of the descriptors in it when its offset
/// told the truth; empty when it lied.
pub struct Cta {
    pub bytes: [u8; 128],
    pub slots: Vec<Kind>,
}

pub fn cta(r: &mut Rng) -> Cta {
    let mut b = [0u8; 128];
    b[0] = 0x02;
    b[1] = if r.one_in(8) { r.byte() } else { 3 };
    b[3] = r.byte();
    let mut at = 4;
    while at < 127 && !r.one_in(4) {
        let (tag, len) = (r.range(0, 7) as u8, r.range(0, 31) as usize);
        b[at] = tag << 5 | len as u8;
        if tag == 7 && at + 1 < 127 {
            b[at + 1] = [0x78, 0x00, 0x01, 0x05, 0x06, r.byte()][r.range(0, 5) as usize];
        }
        let room = len.min(126 - at);
        r.fill(&mut b[at + 1 + (tag == 7) as usize..at + 1 + room.max((tag == 7) as usize)]);
        at += 1 + len;
    }
    let honest = at <= 127 && !r.one_in(4);
    b[2] = if honest { at as u8 } else { r.byte() };
    let mut slots = Vec::new();
    let mut d = b[2] as usize;
    while honest && (4..=127 - 18).contains(&d) {
        let (s, kind) = slot(r);
        b[d..d + 18].copy_from_slice(&s);
        slots.push(kind);
        d += 18;
    }
    let sealed = !r.one_in(16);
    if sealed {
        seal(&mut b);
    }
    // Revision 0 is no CTA-861 block at all, so nothing in it is read.
    let read = honest && sealed && b[1] != 0;
    Cta { bytes: b, slots: if read { slots } else { vec![] } }
}
