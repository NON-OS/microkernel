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
use super::tuning::{
    CONTINUITY_DIV, GAIN_CEIL_X16, GAIN_FLOOR_X16, GAIN_SPEED_DIV, MOTION_CAP, NOMINAL_WIDTH,
};
use super::types::{TouchActions, TouchGesture};

impl TouchGesture {
    /// One finger down at (x, y) since its last report.
    pub(super) fn moved(&mut self, x: u32, y: u32, x_max: u32, act: &mut TouchActions) {
        let dxp = i64::from(x) - self.last_x;
        let dyp = i64::from(y) - self.last_y;
        let den = i64::from(x_max);
        // Discontinuity: re-anchor silently instead of steering the cursor
        // with a torn coordinate. The next clean report resumes motion from
        // here.
        if dxp.abs() > den / CONTINUITY_DIV || dyp.abs() > den / CONTINUITY_DIV {
            self.acc_x = 0;
            self.acc_y = 0;
            return;
        }
        // Real travel counts against the tap; a drag lifts the tap.
        self.tap_travel = self.tap_travel.saturating_add(dxp.abs() + dyp.abs());
        // Speed in nominal pixels per report drives the gain; both axes
        // normalize against the X range so the pad's physical aspect ratio
        // is preserved.
        let speed = (dxp.abs() + dyp.abs()) * NOMINAL_WIDTH / den;
        let gain = (GAIN_FLOOR_X16 + speed / GAIN_SPEED_DIV).min(GAIN_CEIL_X16);
        let mx = step_axis(&mut self.acc_x, dxp, gain, den).clamp(-MOTION_CAP, MOTION_CAP);
        let my = step_axis(&mut self.acc_y, dyp, gain, den).clamp(-MOTION_CAP, MOTION_CAP);
        if mx != 0 || my != 0 {
            act.motion = Some((mx, my));
        }
    }
}

// One axis of the accelerated pad-to-pixel conversion. The accumulator holds
// the remainder in (pad-range * 16) denominator units, so slow motion that
// rounds to zero pixels in one report still adds up across reports.
fn step_axis(acc: &mut i64, d_pad: i64, gain_x16: i64, den_pad: i64) -> i32 {
    let den = den_pad * 16;
    *acc += d_pad * NOMINAL_WIDTH * gain_x16;
    let out = *acc / den;
    *acc -= out * den;
    out as i32
}
