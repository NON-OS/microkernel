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

//! When an event repaints, and what a press does on landing.

use super::build::Controls;
use super::step::Step;

impl<A: Copy + PartialEq> Controls<A> {
    pub(super) fn press(&mut self, was: Option<A>) -> Step<A> {
        self.held = self.lit();
        match self.held {
            Some(a) if self.on_press => Step::Act(a),
            Some(_) => Step::Repaint,
            None => self.redraw(was, false),
        }
    }

    pub(super) fn redraw(&self, was: Option<A>, forced: bool) -> Step<A> {
        if forced || was != self.lit() {
            Step::Repaint
        } else {
            Step::Idle
        }
    }
}
