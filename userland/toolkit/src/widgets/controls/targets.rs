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

//! The hit boxes the app lays its controls out in.

use super::build::Controls;
use crate::widgets::Rect;

/// One hit box: where the control is, what it does, and whether it is live.
#[derive(Clone, Copy, Debug)]
pub(super) struct Target<A> {
    pub(super) at: Rect,
    pub(super) action: A,
    pub(super) live: bool,
}

impl<A: Copy + PartialEq> Controls<A> {
    /// Forgets every box, keeping the pointer, before a new layout.
    pub fn clear(&mut self) {
        self.targets.clear();
    }

    /// Adds a live control at `at` that fires `action`. A box pushed later sits
    /// on top of one pushed earlier where they overlap.
    pub fn push(&mut self, at: Rect, action: A) -> &mut Self {
        self.targets.push(Target { at, action, live: true });
        self
    }

    /// Adds a control at `at` that draws as unavailable: it never hovers,
    /// presses or acts, and it still covers any box beneath it.
    pub fn push_disabled(&mut self, at: Rect, action: A) -> &mut Self {
        self.targets.push(Target { at, action, live: false });
        self
    }

    /// Replaces every box with `targets`, all live, listed bottom to top.
    pub fn set(&mut self, targets: impl IntoIterator<Item = (Rect, A)>) {
        self.clear();
        for (at, action) in targets {
            self.push(at, action);
        }
    }
}
