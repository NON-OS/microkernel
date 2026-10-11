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

//! Taking one event into the report.

use nonos_libc::{input_abs_scale, INPUT_ABS_MAX};

use super::pointer::{Pointer, MAX_BUTTONS};
use super::step::Step;
use crate::init::Axis;

impl Pointer {
    /// An axis value scaled from the device's range; a device that gave no
    /// range for the axis is taken to report in the absolute range itself.
    pub fn set_abs(&mut self, v: i32, axis: Option<Axis>, x: bool) -> Step {
        let a = axis.unwrap_or(Axis { min: 0, max: INPUT_ABS_MAX });
        let scaled = input_abs_scale(v, a.min, a.max);
        if x {
            self.abs.0 = scaled
        } else {
            self.abs.1 = scaled
        }
        self.abs_moved = true;
        Step::Gathered
    }

    pub fn add_rel(&mut self, dx: i32, dy: i32) -> Step {
        self.rel = (self.rel.0.saturating_add(dx), self.rel.1.saturating_add(dy));
        Step::Gathered
    }

    pub fn add_wheel(&mut self, step: i32) -> Step {
        self.wheel = self.wheel.saturating_add(step);
        Step::Gathered
    }

    pub fn button(&mut self, n: u32, down: bool) -> Step {
        if self.n_buttons < MAX_BUTTONS {
            self.buttons[self.n_buttons] = (n, down);
            self.n_buttons += 1;
        }
        Step::Gathered
    }
}
