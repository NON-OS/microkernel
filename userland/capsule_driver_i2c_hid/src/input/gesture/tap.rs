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
use super::tuning::{TAP_MAX_FRAMES, TAP_TRAVEL_DIV};
use super::types::{TouchActions, TouchGesture};

impl TouchGesture {
    /// One finger down: a fresh tap candidate on its rising edge, the motion
    /// since its last report after that (motion.rs).
    pub(super) fn finger(
        &mut self,
        x: u32,
        y: u32,
        x_max: u32,
        button: bool,
        act: &mut TouchActions,
    ) {
        if !self.was_tip {
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
            self.moved(x, y, x_max, act);
        }
        self.last_x = i64::from(x);
        self.last_y = i64::from(y);
        self.was_tip = true;
    }

    /// No finger. On the falling edge, a single finger that touched down and
    /// lifted quickly without travelling is a tap: a left click, down then
    /// up in this report.
    pub(super) fn lift(&mut self, x_max: u32, act: &mut TouchActions) {
        let tap = self.was_tip
            && self.tap_ok
            && self.tip_frames <= TAP_MAX_FRAMES
            && self.tap_travel < i64::from(x_max) / TAP_TRAVEL_DIV;
        if tap {
            act.button_down = true;
            act.button_up = true;
        }
        self.was_tip = false;
    }
}
