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

//! The display range limits descriptor, tag 0xFD (E-EDID 1.4 section
//! 3.10.3.3, Tables 3.26 and 3.27).

use super::descriptor::TAG_RANGE;
use crate::edid::Edid;

/// What the monitor accepts. A rate's limit past 255 is carried by the
/// offset flags in byte 4, which EDID 1.4 added.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeLimits {
    pub v_min_hz: u32,
    pub v_max_hz: u32,
    pub h_min_khz: u32,
    pub h_max_khz: u32,
    /// The largest pixel clock, 0 when the descriptor leaves it out.
    pub max_pixel_khz: u32,
    /// The widest picture a CVT timing may have, 0 for no limit.
    pub max_h_active: u32,
}

impl RangeLimits {
    pub fn decode(d: &[u8]) -> Option<Self> {
        let d = d.get(..18)?;
        let f = d[4];
        // Byte 4: bits 1-0 for the vertical rate, bits 3-2 for the
        // horizontal one. 10 adds 255 to the maximum, 11 to both.
        let add = |bits: u8, max: bool| match (bits & 3, max) {
            (2, true) | (3, _) => 255,
            _ => 0,
        };
        let mut r = RangeLimits {
            v_min_hz: d[5] as u32 + add(f, false),
            v_max_hz: d[6] as u32 + add(f, true),
            h_min_khz: d[7] as u32 + add(f >> 2, false),
            h_max_khz: d[8] as u32 + add(f >> 2, true),
            // Byte 9: the clock in 10 MHz steps.
            max_pixel_khz: d[9] as u32 * 10_000,
            max_h_active: 0,
        };
        // Byte 10 = 04: CVT support follows. Byte 12 bits 7-2 take
        // 0.25 MHz steps off the clock; its bits 1-0 and byte 13 give
        // the widest line in 8 pixel units (Table 3.27).
        if d[10] == 0x04 {
            let less = (d[12] >> 2) as u32 * 250;
            r.max_pixel_khz = r.max_pixel_khz.saturating_sub(less);
            r.max_h_active = 8 * (d[13] as u32 | (d[12] as u32 & 3) << 8);
        }
        if r.v_min_hz > r.v_max_hz || r.h_min_khz > r.h_max_khz {
            return None;
        }
        Some(r)
    }
}

impl Edid<'_> {
    pub fn range_limits(&self) -> Option<RangeLimits> {
        self.display_descriptor(TAG_RANGE).and_then(RangeLimits::decode)
    }
}
