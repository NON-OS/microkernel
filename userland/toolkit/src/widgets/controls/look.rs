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

//! What lies under a point, and the `State` each control draws.

use super::build::Controls;
use crate::widgets::rect::contains;
use crate::widgets::State;

impl<A: Copy + PartialEq> Controls<A> {
    /// The live control under `(x, y)`: the topmost box there, the one pushed
    /// last. None over no box, or when the topmost box there is disabled.
    pub fn at(&self, x: i32, y: i32) -> Option<A> {
        let top = self.targets.iter().rev().find(|t| contains(t.at, x, y))?;
        top.live.then_some(top.action)
    }

    /// The live control under the pointer, if any.
    pub fn lit(&self) -> Option<A> {
        self.pointer.position().and_then(|(x, y)| self.at(x, y))
    }

    /// The hover and press the control for `action` draws, by the same test
    /// as `at`, so it lights exactly where a click on it lands. A disabled
    /// control reports `disabled` and is never hovered or pressed.
    pub fn state(&self, action: A) -> State {
        if self.targets.iter().any(|t| t.action == action && !t.live) {
            return State { disabled: true, ..State::default() };
        }
        self.pointer.state(|x, y| self.at(x, y) == Some(action))
    }

    /// Where the pointer is, if it is over the surface.
    pub fn position(&self) -> Option<(i32, i32)> {
        self.pointer.position()
    }
}
