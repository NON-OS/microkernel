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

use super::types::{
    TouchActions, TouchGesture, CONTINUITY_DIV, GAIN_CEIL_X16, GAIN_FLOOR_X16, GAIN_SPEED_DIV,
    MOTION_CAP, NATURAL_SCROLL, NOMINAL_WIDTH, SCROLL_JUMP_DIV, SCROLL_NOTCHES_PER_PAD,
    TAP_MAX_FRAMES, TAP_TRAVEL_DIV,
};
use crate::hid::TouchSample;

impl TouchGesture {
    /// One decoded frame in, the pointer actions it amounts to out.
    pub fn on_touch(&mut self, s: &TouchSample) -> TouchActions {
        let (x, y, x_max, y_max) = (s.x, s.y, s.x_max, s.y_max);
        let (tip, confidence, contacts, button) = (s.tip, s.confidence, s.contacts, s.button);
        let mut act = TouchActions::default();

        // A physical clickpad press maps to the left button, palm or not: a
        // deliberate press is a deliberate press. Debounced: two consecutive
        // samples must agree before an edge is emitted, so a single-frame bit
        // flip (torn read, misplaced field) can never click.
        if button != self.was_button {
            self.button_run = self.button_run.saturating_add(1);
            if self.button_run >= 2 {
                if button {
                    act.button_down = true;
                } else {
                    act.button_up = true;
                }
                self.was_button = button;
                self.button_run = 0;
            }
        } else {
            self.button_run = 0;
        }

        // Degenerate maxima mean the descriptor's Logical Maximum did not
        // parse; motion math through them is meaningless, so only clicks pass.
        if x_max <= 1 || y_max <= 1 {
            return act;
        }
        let x_max = x_max as u32;
        let y_max = y_max as u32;
        // Torn or short polled reports can carry coordinates beyond the pad's
        // declared range; clamp before any delta or tap math.
        let x = x.min(x_max);
        let y = y.min(y_max);

        // Palm suppression: once a non-confident contact is seen, hold all
        // motion until every contact has lifted. Typing with a palm resting
        // on the pad must not steer the cursor.
        if tip && !confidence {
            self.palm = true;
        }
        if self.palm {
            if !tip && contacts == 0 {
                self.palm = false;
            }
            self.was_tip = false;
            return act;
        }

        // A precision touchpad in hybrid mode sends one finger per report and
        // the frame's contact count only in the first; the others say zero.
        // A report that says zero with its finger down belongs to the frame
        // the last count described.
        if contacts > 0 {
            self.frame_contacts = contacts;
        } else if !tip {
            self.frame_contacts = 0;
        }
        let contacts = if contacts == 0 && tip { self.frame_contacts } else { contacts };

        if contacts >= 2 {
            self.multi_touch = true;
            if let Some(wheel) = self.scroll(s.contact_id, tip, y, y_max) {
                act.wheel = wheel;
            }
            // Two fingers never move the cursor.
            self.was_tip = false;
            return act;
        }
        self.scrolling = false;
        self.scroll_id = None;

        // PTP hybrid reporting alternates which contact a report carries, and
        // only the first report of a frame set carries the true contact count.
        // While several fingers were down, the single-contact frames in
        // between alternate finger positions and would teleport the cursor;
        // hold all movement until every finger has lifted.
        if self.multi_touch {
            if !tip {
                self.multi_touch = false;
            }
            self.was_tip = false;
            return act;
        }

        if tip {
            if !self.was_tip {
                // Rising edge: begin a fresh tap candidate.
                self.acc_x = 0;
                self.acc_y = 0;
                self.tip_frames = 0;
                self.tap_travel = 0;
                self.tap_ok = true;
            }
            self.tip_frames = self.tip_frames.saturating_add(1);
            // A physical clickpad press during the touch is its own click, so it
            // disqualifies the tap and never double-clicks.
            if button {
                self.tap_ok = false;
            }
            if self.was_tip {
                let dxp = i64::from(x) - self.last_x;
                let dyp = i64::from(y) - self.last_y;
                let den = i64::from(x_max);
                // Discontinuity: re-anchor silently instead of steering the
                // cursor with a torn coordinate. The next clean report
                // resumes motion from here.
                if dxp.abs() > den / CONTINUITY_DIV || dyp.abs() > den / CONTINUITY_DIV {
                    self.last_x = i64::from(x);
                    self.last_y = i64::from(y);
                    self.acc_x = 0;
                    self.acc_y = 0;
                    self.was_tip = tip;
                    return act;
                }
                // Real travel counts against the tap; a drag lifts the tap.
                self.tap_travel = self.tap_travel.saturating_add(dxp.abs() + dyp.abs());
                // Speed in nominal pixels per report drives the gain; both
                // axes normalize against the X range so the pad's physical
                // aspect ratio is preserved.
                let speed = (dxp.abs() + dyp.abs()) * NOMINAL_WIDTH / den;
                let gain = (GAIN_FLOOR_X16 + speed / GAIN_SPEED_DIV).min(GAIN_CEIL_X16);
                let mx = step_axis(&mut self.acc_x, dxp, gain, den).clamp(-MOTION_CAP, MOTION_CAP);
                let my = step_axis(&mut self.acc_y, dyp, gain, den).clamp(-MOTION_CAP, MOTION_CAP);
                if mx != 0 || my != 0 {
                    act.motion = Some((mx, my));
                }
            }
            self.last_x = i64::from(x);
            self.last_y = i64::from(y);
        } else if self.was_tip {
            // Falling edge: a single finger that touched down and lifted
            // quickly without travelling is a tap. Emit a left click (down then
            // up in this report) so a quick tap registers as a left click.
            if self.tap_ok
                && self.tip_frames <= TAP_MAX_FRAMES
                && self.tap_travel < i64::from(x_max) / TAP_TRAVEL_DIV
            {
                act.button_down = true;
                act.button_up = true;
            }
        }
        self.was_tip = tip;
        act
    }
}

// One axis of the accelerated pad-to-pixel conversion. The accumulator holds
// the remainder in (pad-range * 16) denominator units, so slow motion that
// rounds to zero pixels in one report still adds up across reports.
impl TouchGesture {
    /// One report of a two-finger touch. The scroll follows one finger: the
    /// first one seen, by its contact identifier when the pad gives one, so
    /// the reports of the other finger in hybrid mode neither move it nor
    /// re-anchor it. Whole notches are posted and the rest is kept for the
    /// next report.
    fn scroll(&mut self, id: Option<u32>, tip: bool, y: u32, y_max: u32) -> Option<i32> {
        if !self.scrolling {
            if tip {
                self.scrolling = true;
                self.scroll_id = id;
                self.scroll_y = y;
            }
            return None;
        }
        let mine = match (self.scroll_id, id) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        };
        if !mine || !tip {
            return None;
        }
        let dy = y as i32 - self.scroll_y as i32;
        if dy.unsigned_abs() > y_max / SCROLL_JUMP_DIV {
            self.scroll_y = y;
            return None;
        }
        let step = (y_max / SCROLL_NOTCHES_PER_PAD).max(1) as i32;
        let notches = dy / step;
        if notches == 0 {
            return None;
        }
        self.scroll_y = (self.scroll_y as i32 + notches * step) as u32;
        // Fingers moving down the pad (y growing) scroll the view down,
        // which is a negative wheel, unless natural scrolling is on.
        Some(if NATURAL_SCROLL { notches } else { -notches })
    }
}

fn step_axis(acc: &mut i64, d_pad: i64, gain_x16: i64, den_pad: i64) -> i32 {
    let den = den_pad * 16;
    *acc += d_pad * NOMINAL_WIDTH * gain_x16;
    let out = *acc / den;
    *acc -= out * den;
    out as i32
}
