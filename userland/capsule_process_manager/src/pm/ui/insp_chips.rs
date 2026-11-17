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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::Tone;

use crate::pm::format_caps::CAP_TABLE;

use super::insp_chip::{chip, chip_w, more, more_w};
use super::metrics::metrics;
use super::tint::cap_tint;

// The decoded grant list, wrapped by measured width rather than by a glyph
// count, because the body face is proportional. It stops above `bottom`, where
// the actions begin: grants that would not fit are counted in a last "+N more"
// chip on the final row that does, never drawn under the buttons.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, bottom: u32, caps: u64) {
    let total = CAP_TABLE.iter().filter(|(bit, _)| caps & bit != 0).count() as u32;
    let (chip_h, chip_gap) = (metrics().chip_h, metrics().chip_gap);
    let (mut cx, mut cy, mut shown) = (x, y, 0u32);
    for &(bit, label) in CAP_TABLE.iter().filter(|(bit, _)| caps & bit != 0) {
        let cw = chip_w(fb, label);
        let wrap = cx > x && cx + cw > x + w;
        let (nx, ny) = if wrap { (x, cy + chip_h + chip_gap) } else { (cx, cy) };
        let after = total - shown - 1;
        let last_row = ny + chip_h * 2 + chip_gap > bottom;
        let squeezed = after > 0 && last_row && nx + cw + chip_gap + more_w(fb, after) > x + w;
        if ny + chip_h > bottom || squeezed {
            break;
        }
        chip(fb, nx, ny, label, cap_tint(bit));
        (cx, cy, shown) = (nx + cw + chip_gap, ny, shown + 1);
    }
    if shown < total && cy + chip_h <= bottom {
        let mut buf = [0u8; 16];
        let n = more(total - shown, &mut buf);
        chip(fb, cx, cy, &buf[..n], Tone::Plain);
    }
}
