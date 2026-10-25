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
//! A workspace's floating toggle and its windows in order.

use alloc::vec::Vec;

use super::types::Win;
use super::workspace::Workspace;

impl Workspace {
    /// Float a tiled window, or tile a floating one; it keeps focus.
    pub fn toggle_float(&mut self, w: Win) -> bool {
        let floated = self.floating.contains(&w);
        if !self.take(w) {
            return false;
        }
        if floated {
            self.add(w);
        } else {
            self.floating.push(w);
        }
        self.focus = Some(w);
        true
    }

    /// The tiled windows in order, then the floating ones.
    pub fn windows(&self) -> Vec<Win> {
        let mut out = Vec::new();
        if let Some(t) = &self.tree {
            t.leaves(&mut out);
        }
        out.extend(self.floating.iter().copied());
        out
    }
}
