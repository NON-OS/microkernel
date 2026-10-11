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

//! CVT reduced blanking, versions 1 and 2 (VESA CVT 1.2 section 3.5), for
//! panels that need no CRT retrace time. Fixed horizontal blanking; the
//! vertical blanking covers at least 460 microseconds.

use super::aspect::{v_sync, CELL};
use crate::make::{timing, Axis};
use nonos_edid::Detailed;

const RB_MIN_V_BLANK: u128 = 460;

/// Lines of vertical blanking: enough for the minimum time, and at least
/// `least` lines.
fn vbi(height: u32, refresh_mhz: u32, least: u32) -> Option<u32> {
    let r = refresh_mhz as u128;
    let num = 1_000_000_000u128.checked_sub(RB_MIN_V_BLANK * r).filter(|&n| n > 0)?;
    let den = r * height as u128;
    Some(((RB_MIN_V_BLANK * den / num) as u32 + 1).max(least))
}

/// Version 1: 160 pixels of blanking (48 front, 32 sync, 80 back), 3
/// lines of front porch, the aspect ratio's sync, back porch 6 at least;
/// the clock in 0.25 MHz steps. Sync is +H -V.
pub fn reduced(width: u32, height: u32, refresh_mhz: u32) -> Option<Detailed> {
    let w = width / CELL * CELL;
    if w == 0 || height == 0 {
        return None;
    }
    let vs = v_sync(w, height);
    let vbi = vbi(height, refresh_mhz, 3 + vs + 6)?;
    let v_total = (height + vbi) as u128;
    let quarters = refresh_mhz as u128 * v_total * (w + 160) as u128 * 4 / 1_000_000_000;
    let h = Axis::new(w, 48, 32, 80, true);
    let v = Axis::new(height, 3, vs, vbi - 3 - vs, false);
    Some(timing(u32::try_from(250 * quarters).ok().filter(|&c| c > 0)?, h, v))
}

/// Version 2: 80 pixels of blanking (8 front, 32 sync, 40 back), 8 lines
/// of sync, back porch 6, front porch 1 at least; any width; the clock in
/// 1 kHz steps. Sync is +H -V.
pub fn reduced_v2(width: u32, height: u32, refresh_mhz: u32) -> Option<Detailed> {
    if width == 0 || height == 0 {
        return None;
    }
    let vbi = vbi(height, refresh_mhz, 1 + 8 + 6)?;
    let v_total = (height + vbi) as u128;
    let khz = refresh_mhz as u128 * v_total * (width + 80) as u128 / 1_000_000;
    let h = Axis::new(width, 8, 32, 40, true);
    let v = Axis::new(height, vbi - 8 - 6, 8, 6, false);
    Some(timing(u32::try_from(khz).ok().filter(|&c| c > 0)?, h, v))
}
