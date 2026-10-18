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
use super::types::{TouchActions, TouchGesture};
use crate::hid::TouchSample;

impl TouchGesture {
    /// One decoded frame in, the pointer actions it amounts to out: the
    /// clickpad button (debounce.rs), then a palm (palm.rs), two fingers or
    /// more (several.rs), and one finger moving (motion.rs) or lifting
    /// (tap.rs).
    pub fn on_touch(&mut self, s: &TouchSample) -> TouchActions {
        let mut act = TouchActions::default();
        self.debounce(s.button, &mut act);

        // Degenerate maxima mean the descriptor's Logical Maximum did not
        // parse; motion math through them is meaningless, so only clicks pass.
        if s.x_max <= 1 || s.y_max <= 1 {
            return act;
        }
        let (x_max, y_max) = (s.x_max as u32, s.y_max as u32);
        // Torn or short polled reports can carry coordinates beyond the pad's
        // declared range; clamp before any delta or tap math.
        let (x, y) = (s.x.min(x_max), s.y.min(y_max));

        if self.palm_held(s.tip, s.confidence, s.contacts) || self.several(s, y, y_max, &mut act) {
            self.was_tip = false;
            return act;
        }
        if s.tip {
            self.finger(x, y, x_max, s.button, &mut act);
        } else {
            self.lift(x_max, &mut act);
        }
        act
    }
}
