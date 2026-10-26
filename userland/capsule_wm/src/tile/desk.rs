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
//! The nine workspaces and the one shown. Switching says which windows to
//! hide and which to show; a window sent elsewhere keeps floating if it
//! floated.

use alloc::vec::Vec;

use super::types::Win;
use super::workspace::Workspace;

pub const SPACES: usize = 9;

pub struct Desk {
    pub spaces: [Workspace; SPACES],
    pub shown: usize,
}

impl Desk {
    pub fn new() -> Self {
        Self { spaces: core::array::from_fn(|_| Workspace::new()), shown: 0 }
    }

    /// A new window opens on the workspace shown.
    pub fn open(&mut self, w: Win) {
        self.spaces[self.shown].add(w);
    }

    /// A window closed, wherever it was.
    pub fn close(&mut self, w: Win) {
        self.spaces.iter_mut().for_each(|s| _ = s.take(w));
    }

    /// Show workspace `n`: the windows to hide, then those to show. None
    /// for one that does not exist or is already shown.
    pub fn switch(&mut self, n: usize) -> Option<(Vec<Win>, Vec<Win>)> {
        if n >= SPACES || n == self.shown {
            return None;
        }
        let hide = self.spaces[self.shown].windows();
        self.shown = n;
        Some((hide, self.spaces[n].windows()))
    }
}

impl Default for Desk {
    fn default() -> Self {
        Self::new()
    }
}
