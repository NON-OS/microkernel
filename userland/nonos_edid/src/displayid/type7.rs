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

//! Type VII detailed timings, DisplayID 2.0 data block tag 0x22 (section
//! 4.4.1). Each field is stored less one; the clock is in 1 kHz units in
//! three bytes. The descriptor is 20 bytes; bits 6-4 of the block's
//! revision byte add bytes to it (unverified against the document: read
//! as the extra length so a later layout is skipped rather than misread).

use super::section::Section;
use crate::base::dtd::Detailed;
use crate::source::Source;
use crate::timing::Timing;

pub const TAG_TYPE7: u8 = 0x22;

/// One descriptor as a detailed timing and its preferred bit.
/// One 20 byte record, its clock counted in `unit_khz` steps (type VII
/// counts single kHz, type I tens). None when it is no sound timing.
pub fn decode(d: &[u8], unit_khz: u32) -> Option<(Detailed, bool)> {
    let d = d.get(..20)?;
    let w = |i: usize| u16::from_le_bytes([d[i], d[i + 1]]) as u32;
    let clock = (d[0] as u32 | (d[1] as u32) << 8 | (d[2] as u32) << 16) + 1;
    // Byte 3: bit 7 preferred, bit 4 interlaced, the rest stereo and
    // aspect ratio, which do not change the timing.
    let t = Detailed {
        pixel_khz: clock.saturating_mul(unit_khz),
        h_active: w(4) + 1,
        h_blank: w(6) + 1,
        h_front: (w(8) & 0x7FFF) + 1,
        h_positive: w(8) & 0x8000 != 0,
        h_sync: w(10) + 1,
        v_active: w(12) + 1,
        v_blank: w(14) + 1,
        v_front: (w(16) & 0x7FFF) + 1,
        v_positive: w(16) & 0x8000 != 0,
        v_sync: w(18) + 1,
        interlaced: d[3] & 0x10 != 0,
        width_mm: 0,
        height_mm: 0,
    };
    t.sound().then_some((t, d[3] & 0x80 != 0))
}

/// Every type VII timing in a DisplayID 2.x section.
pub(crate) fn each(s: Section<'_>, f: &mut dyn FnMut(Timing)) {
    if !s.v2() {
        return;
    }
    for b in s.blocks().filter(|b| b.tag == TAG_TYPE7) {
        let size = 20 + ((b.revision >> 4) & 7) as usize;
        for (t, preferred) in b.body.chunks_exact(size).filter_map(|r| decode(r, 1)) {
            let mut t = Timing::detailed(t, Source::DisplayId);
            t.preferred = preferred;
            f(t);
        }
    }
}
