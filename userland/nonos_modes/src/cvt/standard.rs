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

//! CVT standard blanking (VESA CVT 1.2 section 3.4, the CRT timing
//! formula), progressive and without margins, in exact integer arithmetic:
//! the standard's own reals are kept as fractions, so every floor lands
//! where the standard's spreadsheet puts it.

use super::aspect::{v_sync, CELL};
use crate::make::{timing, Axis};
use nonos_edid::Detailed;

/// Microseconds of vertical sync plus back porch, at least.
const MIN_VSYNC_BP: u128 = 550;
/// Lines of front porch and of back porch, at least.
const MIN_V_PORCH: u32 = 3;
const MIN_V_BPORCH: u32 = 6;

pub fn standard(width: u32, height: u32, refresh_mhz: u32) -> Option<Detailed> {
    let w = width / CELL * CELL;
    if w == 0 || height == 0 || refresh_mhz == 0 {
        return None;
    }
    let vs = v_sync(w, height);
    let r = refresh_mhz as u128;
    // The estimated line period, in microseconds, is num / den.
    let num = 1_000_000_000u128.checked_sub(MIN_VSYNC_BP * r).filter(|&n| n > 0)?;
    let den = r * (height + MIN_V_PORCH) as u128;
    let vsbp = ((MIN_VSYNC_BP * den / num) as u32 + 1).max(vs + MIN_V_BPORCH);
    // The ideal duty cycle C' - M' * period / 1000 with C' 30 and M'
    // 300, as d_num / d_den percent, held at 20 at least.
    let (d_num, d_den) = match (300 * den).checked_sub(3 * num) {
        Some(n) if n >= 200 * den => (n, 10 * den),
        _ => (20, 1),
    };
    let h_blank =
        (w as u128 * d_num / ((100 * d_den - d_num) * 2 * CELL as u128)) as u32 * 2 * CELL;
    let total = w + h_blank;
    // The clock in 0.25 MHz steps: total / period, floored to a step.
    let clock = 250 * (4 * total as u128 * den / num);
    // Sync is 8 percent of the line, in whole cells.
    let h_sync = total / 100 * CELL;
    let back = h_blank / 2;
    let h = Axis::new(w, h_blank - h_sync - back, h_sync, back, false);
    let v = Axis::new(height, MIN_V_PORCH, vs, vsbp - vs, true);
    Some(timing(u32::try_from(clock).ok().filter(|&c| c > 0)?, h, v))
}
