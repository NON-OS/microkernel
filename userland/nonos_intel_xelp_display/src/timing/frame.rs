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
//! How long one frame of a timing lasts, for the waits that Vol 12 gives
//! in frame times.

use nonos_edid::Detailed;

/// One frame of `t` in microseconds, rounded up, and at most u64::MAX; None
/// for a timing with no pixel clock. The sums and the product are taken in
/// 128 bits, so no field a monitor or caller sends can overflow them.
pub fn frame_us(t: &Detailed) -> Option<u64> {
    let total_h = t.h_active as u128 + t.h_blank as u128;
    let total_v = t.v_active as u128 + t.v_blank as u128;
    let khz = t.pixel_khz as u128;
    if khz == 0 {
        return None;
    }
    // A pixel lasts 1000 / khz microseconds.
    let us = (total_h * total_v * 1000).div_ceil(khz);
    Some(u64::try_from(us).unwrap_or(u64::MAX))
}
