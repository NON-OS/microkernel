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

//! Timings named by DMT ID: DisplayID 1.3 type IV (tag 0x06) and 2.0 type
//! VIII (tag 0x23), each a list of one-byte codes when the revision byte's
//! two high bits are 0 (DMT; VIC and HDMI VIC codes are left to CTA) and,
//! for type VIII, bit 3 is clear (two-byte codes are skipped). Recalled,
//! unverified. The IDs are looked up in nonos_modes' DMT table, read as
//! text here so the two crates share one transcription.

use super::section::Section;
use crate::source::Source;
use crate::timing::Timing;

pub const TAG_TYPE4: u8 = 0x06;
pub const TAG_TYPE8: u8 = 0x23;
const DMT: &str = include_str!("../../../nonos_modes/data/dmt.txt");

/// The DMT mode with `id`: its size, rate and whether it is reduced
/// blanking; None for an ID the table does not list.
pub fn lookup(id: u8) -> Option<(u32, u32, u32, bool)> {
    let row = DMT.lines().filter(|l| !l.starts_with('#')).find(|l| {
        l.split_whitespace().next().and_then(|c| u8::from_str_radix(c, 16).ok()) == Some(id)
    })?;
    let field = |i: usize| row.split_whitespace().nth(i);
    let n = |i: usize| field(i)?.parse::<u32>().ok();
    Some((n(1)?, n(2)?, n(3)?, matches!(field(12)?, "r" | "2")))
}

pub(crate) fn each(s: Section<'_>, f: &mut dyn FnMut(Timing)) {
    let tag = if s.v2() { TAG_TYPE8 } else { TAG_TYPE4 };
    for b in s.blocks().filter(|b| b.tag == tag && b.revision & 0xC0 == 0) {
        if s.v2() && b.revision & 0x08 != 0 {
            continue;
        }
        for (w, h, hz, rb) in b.body.iter().filter_map(|&id| lookup(id)) {
            let mut t = Timing::nominal(w, h, hz, Source::DisplayId);
            t.reduced_blanking = rb;
            f(t);
        }
    }
}
