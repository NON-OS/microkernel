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

//! The answers a display driver asks for most, built on the walk.

use crate::base::dtd::Detailed;
use crate::edid::Edid;
use crate::source::Source;
use crate::timing::Timing;

impl Edid<'_> {
    /// The preferred timing: the base block's first detailed timing,
    /// else a DisplayID timing marked preferred, else the first detailed
    /// timing of a CTA extension. A panel whose base block has only
    /// display descriptors (DisplayID 2.0 laptops do this) still answers.
    pub fn preferred(&self) -> Option<Detailed> {
        let mut first_cta = None;
        let mut found = None;
        self.for_each_timing(&mut |t: Timing| {
            if found.is_some() {
                return;
            }
            match (t.source, t.preferred, t.detail) {
                (Source::Detailed | Source::DisplayId, true, d @ Some(_)) => found = d,
                (Source::CtaDetailed, _, d @ Some(_)) if first_cta.is_none() => first_cta = d,
                _ => {}
            }
        });
        found.or(first_cta)
    }

    /// The physical size in mm packed as `w | h << 16`, saturated to 16
    /// bits each, as the display wire's CAPS reply carries it; 0 unknown.
    pub fn physical_mm(&self) -> u32 {
        let (w, h) = self.size_mm();
        if w == 0 || h == 0 {
            return 0;
        }
        w.min(0xFFFF) | h.min(0xFFFF) << 16
    }
}
