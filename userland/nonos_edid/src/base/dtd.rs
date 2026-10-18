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

/// One detailed timing: E-EDID 1.4 section 3.10.2 (Table 3.21), a CTA-861
/// extension's own descriptors in the same layout, or a DisplayID 2.0
/// type VII descriptor. Blanking includes the porches and sync; a field's
/// lines for an interlaced timing, as the descriptor gives them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Detailed {
    pub pixel_khz: u32,
    pub h_active: u32,
    pub h_blank: u32,
    pub h_front: u32,
    pub h_sync: u32,
    pub v_active: u32,
    pub v_blank: u32,
    pub v_front: u32,
    pub v_sync: u32,
    pub h_positive: bool,
    pub v_positive: bool,
    pub interlaced: bool,
    pub width_mm: u32,
    pub height_mm: u32,
}

impl Detailed {
    /// The picture's width in pixels.
    pub fn width(&self) -> u32 {
        self.h_active
    }

    /// The frame's height: an interlaced descriptor counts one field's
    /// lines (Table 3.21's note on byte 17 bit 7), so the frame has two.
    pub fn height(&self) -> u32 {
        self.v_active.saturating_mul(1 + self.interlaced as u32)
    }

    pub fn h_total(&self) -> u32 {
        self.h_active.saturating_add(self.h_blank)
    }

    pub fn v_total(&self) -> u32 {
        self.v_active.saturating_add(self.v_blank)
    }

    /// Frames (or fields, interlaced) per second, in millihertz. A
    /// hostile descriptor with tiny totals is clamped, not wrapped.
    pub fn refresh_mhz(&self) -> u32 {
        let total = self.h_total() as u64 * self.v_total() as u64;
        if total == 0 {
            return 0;
        }
        let mhz = self.pixel_khz as u64 * 1_000_000 / total;
        mhz.min(u32::MAX as u64) as u32
    }
}
