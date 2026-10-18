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

//! One report's worth of pointer state. Positions, motion and the wheel
//! accumulate until SYN_REPORT, and buttons queue in order, so a click that
//! arrives with a move lands where the pointer went, not where it was.

use super::buttons::{button_number, gear_step};
use super::pointer::Pointer;
use super::raw::Raw;
use super::step::Step;
use crate::constants::evdev::{
    ABS_X, ABS_Y, BTN_MISC, EV_ABS, EV_KEY, EV_REL, EV_SYN, REL_WHEEL, REL_X, REL_Y, SYN_DROPPED,
};
use crate::init::Axes;

pub struct Report {
    pending: Pointer,
}

impl Report {
    pub const fn new() -> Self {
        Self { pending: Pointer::empty() }
    }

    pub fn feed(&mut self, ev: Raw, axes: &Axes) -> Step {
        let p = &mut self.pending;
        match (ev.kind, ev.code) {
            (EV_SYN, SYN_DROPPED) => {
                *p = Pointer::empty();
                Step::Gathered
            }
            (EV_SYN, _) => Step::Flush,
            (EV_ABS, ABS_X) => p.set_abs(ev.value, axes.x, true),
            (EV_ABS, ABS_Y) => p.set_abs(ev.value, axes.y, false),
            (EV_REL, REL_X) => p.add_rel(ev.value, 0),
            (EV_REL, REL_Y) => p.add_rel(0, ev.value),
            (EV_REL, REL_WHEEL) => p.add_wheel(ev.value),
            (EV_KEY, code) if code < BTN_MISC => Step::Key(code, ev.value),
            (EV_KEY, code) => match (button_number(code), gear_step(code, ev.value)) {
                (Some(n), _) if ev.value == 0 || ev.value == 1 => p.button(n, ev.value == 1),
                (_, Some(step)) => p.add_wheel(step),
                _ => Step::Ignored,
            },
            _ => Step::Ignored,
        }
    }

    /// The gathered report, leaving an empty one; the position is kept, as
    /// a report that moves one axis leaves the other where it was.
    pub fn take(&mut self) -> Pointer {
        let out = self.pending;
        self.pending = Pointer::after(&out);
        out
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}
