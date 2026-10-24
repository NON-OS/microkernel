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
//! Timings the tests start from, written out from their standards.

use nonos_edid::Detailed;

/// 1920x1080 at 60 Hz, CTA-861 VIC 16: 148.5 MHz, 280 blank (88 front, 44
/// sync), 45 lines of blank (4 front, 5 sync), both syncs positive.
pub fn cta_1080p() -> Detailed {
    timing(148_500, [1920, 280, 88, 44], [1080, 45, 4, 5])
}

/// 800x600 at 60 Hz, VESA DMT: 40 MHz, 256 blank (40, 128), 28 (1, 4).
pub fn dmt_800x600() -> Detailed {
    timing(40_000, [800, 256, 40, 128], [600, 28, 1, 4])
}

/// A timing from its horizontal and vertical [active, blank, front, sync].
pub fn timing(khz: u32, h: [u32; 4], v: [u32; 4]) -> Detailed {
    Detailed {
        pixel_khz: khz,
        h_active: h[0],
        h_blank: h[1],
        h_front: h[2],
        h_sync: h[3],
        v_active: v[0],
        v_blank: v[1],
        v_front: v[2],
        v_sync: v[3],
        h_positive: true,
        v_positive: true,
        interlaced: false,
        width_mm: 0,
        height_mm: 0,
    }
}
