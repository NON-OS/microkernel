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

//! Whether a timing is one the sink and the source can carry: the pixel
//! clock against a limit, and every rate against the monitor's range
//! limits descriptor (E-EDID 1.4 section 3.10.3.3).

use nonos_edid::{Detailed, RangeLimits};

/// The clock is set and within `max_khz` (0 for no limit).
pub fn clock_ok(t: &Detailed, max_khz: u32) -> bool {
    t.pixel_khz != 0 && (max_khz == 0 || t.pixel_khz <= max_khz)
}

/// The line rate in Hz, the frame rate in millihertz.
fn rates(t: &Detailed) -> Option<(u64, u64)> {
    let (ht, vt) = (t.h_total() as u64, t.v_total() as u64);
    if ht == 0 || vt == 0 {
        return None;
    }
    let line_hz = t.pixel_khz as u64 * 1000 / ht;
    Some((line_hz, line_hz * 1000 / vt))
}

/// The timing's line and frame rates, clock and width fall inside the
/// descriptor's limits. The descriptor's rates are whole Hz and kHz, so
/// a rate is held to the limit widened by the unit it was rounded to.
pub fn in_range(t: &Detailed, r: &RangeLimits) -> bool {
    let Some((line_hz, frame_mhz)) = rates(t) else { return false };
    let v = r.v_min_hz as u64 * 1000..(r.v_max_hz as u64 + 1) * 1000;
    let h = (r.h_min_khz as u64).saturating_sub(1) * 1000..(r.h_max_khz as u64 + 1) * 1000;
    let width = r.max_h_active == 0 || t.h_active <= r.max_h_active;
    v.contains(&frame_mhz) && h.contains(&line_hz) && clock_ok(t, r.max_pixel_khz) && width
}
