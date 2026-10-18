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

//! Timings named by size and rate, for a CVT formula to fill in: DisplayID
//! 1.3 type III (tag 0x05, three bytes: options, width/8 less one, then
//! interlace and rate less one) and 2.0 type IX (tag 0x24, six bytes:
//! options, width and height less one, rate less one). In the options the
//! formula is 0 CVT or 1 or 2 reduced blanking (type IX's low three bits,
//! type III's bits 6-4); bit 7 is preferred; type III's low nibble is the
//! aspect ratio. Layouts as
//! recalled from the VESA documents, unverified.

use super::section::Section;
use crate::source::Source;
use crate::timing::Timing;

pub const TAG_TYPE3: u8 = 0x05;
pub const TAG_TYPE9: u8 = 0x24;

/// Type III's aspect ratios, height over width, by the low nibble.
const ASPECT: [(u32, u32); 8] =
    [(1, 1), (4, 5), (3, 4), (9, 15), (9, 16), (10, 16), (27, 64), (135, 256)];

fn timing(w: u32, h: u32, hz: u32, preferred: bool, formula: u8, interlaced: bool) -> Timing {
    let mut t = Timing::nominal(w, h, hz, Source::DisplayId);
    (t.preferred, t.interlaced, t.reduced_blanking) =
        (preferred, interlaced, matches!(formula, 1 | 2));
    t
}

pub fn type3(d: &[u8]) -> Option<Timing> {
    let (num, den) = *ASPECT.get((d[0] & 0xF) as usize)?;
    let w = (d[1] as u32 + 1) * 8;
    let hz = (d[2] & 0x7F) as u32 + 1;
    Some(timing(w, w * num / den, hz, d[0] & 0x80 != 0, (d[0] >> 4) & 7, d[2] & 0x80 != 0))
}

pub fn type9(d: &[u8]) -> Option<Timing> {
    let w = u16::from_le_bytes([d[1], d[2]]) as u32 + 1;
    let h = u16::from_le_bytes([d[3], d[4]]) as u32 + 1;
    let formula = d[0] & 7;
    (formula <= 2).then(|| timing(w, h, d[5] as u32 + 1, d[0] & 0x80 != 0, formula, false))
}

/// A descriptor's reader.
type Reader = fn(&[u8]) -> Option<Timing>;

pub(crate) fn each(s: Section<'_>, f: &mut dyn FnMut(Timing)) {
    let (tag, size, read): (u8, usize, Reader) =
        if s.v2() { (TAG_TYPE9, 6, type9) } else { (TAG_TYPE3, 3, type3) };
    for b in s.blocks().filter(|b| b.tag == tag) {
        b.body.chunks_exact(size).filter_map(read).for_each(&mut *f);
    }
}
