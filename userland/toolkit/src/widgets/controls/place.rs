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

//! Laying a control out and drawing it in one step.

use super::build::Controls;
use crate::widgets::{Rect, State};

impl<A: Copy + PartialEq> Controls<A> {
    /// Pushes a live box at `at` for `action` and returns the `State` to draw
    /// it with, so a paint lays out, hit-tests and draws from one rectangle:
    /// `Button::new("Save").state(c.place(at, Save)).draw(fb, at)`. Where a
    /// later box overlaps this one, push them all and then ask `state`.
    pub fn place(&mut self, at: Rect, action: A) -> State {
        self.push(at, action);
        self.state(action)
    }

    /// Pushes a disabled box at `at` for `action` and returns its `State`,
    /// which is `disabled` and never hovered or pressed.
    pub fn place_disabled(&mut self, at: Rect, action: A) -> State {
        self.push_disabled(at, action);
        self.state(action)
    }
}
