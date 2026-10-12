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

//! Whether firmware's plane holds red in the lowest byte: DSPCNTR's
//! format 1110b "RGBX 8:8:8:8" on Haswell and Broadwell, PLANE_CTL's RGB
//! Color Order bit from Gen9 (positions as `plane_kind` cites them). The
//! driver's own surfaces are always blue first (ARGB8888's order), so
//! firmware's picture is swapped as it is copied in.

use crate::engine::gen::PlaneKind;
use crate::engine::plane_read::PlaneState;

pub fn red_first(p: &PlaneState) -> bool {
    match p.kind {
        PlaneKind::Primary => p.ctl & p.kind.format_mask() == 0xE << 26,
        _ => p.ctl & p.kind.order_bit() != 0,
    }
}

/// Swap red and blue in a row of 32-bit pixels, in place.
pub fn swap_red_blue(row: &mut [u8]) {
    for px in row.chunks_exact_mut(4) {
        px.swap(0, 2);
    }
}
