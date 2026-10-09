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

use alloc::vec::Vec;

use super::axis::{Axis, UNIT};

const ONE: u64 = 1 << 16;

// Bilinear taps between the two source centres around each destination centre.
pub fn linear(src: u32, off: u64, span: u64, dst: u32) -> Axis {
    let mut start = Vec::with_capacity(dst as usize);
    let mut weight = Vec::with_capacity(dst as usize * 2);
    // Centres of the first and last source cells inside the window.
    let lo = (off / ONE) * ONE;
    let hi = ((off + span - 1) / ONE).min(src as u64 - 1) * ONE;
    for i in 0..dst as u64 {
        // Destination centre in source space, less half a cell to reach
        // the coordinate system of cell centres.
        let c = off + ((2 * i + 1) * span) / (2 * dst as u64);
        let s = c.saturating_sub(ONE / 2).clamp(lo, hi);
        let base = s / ONE;
        let frac = ((s % ONE) * UNIT as u64 / ONE) as u16;
        let next_ok = base * ONE < hi;
        start.push(base as u32);
        if next_ok {
            weight.push(UNIT as u16 - frac);
            weight.push(frac);
        } else {
            weight.push(UNIT as u16);
            weight.push(0);
        }
    }
    Axis { start, weight, taps: 2 }
}
