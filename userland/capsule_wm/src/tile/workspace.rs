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
//! One workspace: its tiled windows in a tree, its floating windows beside
//! it, the window that has focus, and the split the next window opens in.

use alloc::vec::Vec;

use super::insert::insert;
use super::node::Node;
use super::remove::remove;
use super::types::{Split, Win};

pub struct Workspace {
    pub tree: Option<Node>,
    pub floating: Vec<Win>,
    pub focus: Option<Win>,
    pub split: Split,
}

impl Workspace {
    pub const fn new() -> Self {
        Self { tree: None, floating: Vec::new(), focus: None, split: Split::Across }
    }

    pub fn tiled(&self, w: Win) -> bool {
        self.tree.as_ref().is_some_and(|t| t.holds(w))
    }

    pub fn holds(&self, w: Win) -> bool {
        self.tiled(w) || self.floating.contains(&w)
    }

    /// Tile `w` beside the focused tiled window, and focus it.
    pub fn add(&mut self, w: Win) {
        if self.holds(w) {
            return;
        }
        let at = self.focus.filter(|f| self.tiled(*f));
        self.tree = Some(insert(self.tree.take(), at, w, self.split));
        self.focus = Some(w);
    }

    /// Take `w` out, tiled or floating; focus passes to the first window
    /// left. False when it was not here.
    pub fn take(&mut self, w: Win) -> bool {
        let had = self.holds(w);
        self.tree = self.tree.take().and_then(|t| remove(t, w));
        self.floating.retain(|f| *f != w);
        if self.focus == Some(w) {
            self.focus = self.windows().first().copied();
        }
        had
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}
