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

//! The pointer events an app passes in, each answered with a `Step`.

use super::build::Controls;
use super::step::Step;

impl<A: Copy + PartialEq> Controls<A> {
    /// The pointer moved to `(x, y)`. Repaints only when the lit control
    /// changed.
    pub fn moved(&mut self, x: i32, y: i32) -> Step<A> {
        let was = self.lit();
        self.pointer.moved(x, y);
        self.redraw(was, false)
    }

    /// A button went down at `(x, y)`. Repaints when it lands on a live control
    /// or leaves one; acts at once when the controls act on press.
    pub fn pressed(&mut self, x: i32, y: i32) -> Step<A> {
        let was = self.lit();
        self.pointer.pressed(x, y);
        self.press(was)
    }

    /// A finger came down at `(x, y)`: a press that hovers nothing.
    pub fn touched(&mut self, x: i32, y: i32) -> Step<A> {
        let was = self.lit();
        self.pointer.touched(x, y);
        self.press(was)
    }

    /// The button or finger came up. Acts when it comes up over the control
    /// the press began on, unless the controls already acted on press.
    pub fn released(&mut self) -> Step<A> {
        let held = self.held.take();
        let over = self.lit();
        self.pointer.released();
        match held {
            Some(a) if !self.on_press && over == Some(a) => Step::Act(a),
            _ => self.redraw(over, held.is_some()),
        }
    }

    /// The pointer left the surface: nothing hovers and a held press is
    /// dropped without acting.
    pub fn left(&mut self) -> Step<A> {
        let was = self.lit();
        let held = self.held.take().is_some();
        self.pointer.left();
        self.redraw(was, held)
    }
}
