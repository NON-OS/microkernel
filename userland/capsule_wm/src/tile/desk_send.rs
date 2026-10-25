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
//! A window sent to another workspace; it keeps floating if it floated.

use super::desk::{Desk, SPACES};
use super::types::Win;

impl Desk {
    /// Move `w` to workspace `n`. False when there is no such workspace,
    /// `w` is nowhere, or it is there already.
    pub fn send(&mut self, w: Win, n: usize) -> bool {
        let Some(from) = self.spaces.iter().position(|s| s.holds(w)) else { return false };
        if n >= SPACES || n == from {
            return false;
        }
        let floated = self.spaces[from].floating.contains(&w);
        self.spaces[from].take(w);
        let to = &mut self.spaces[n];
        to.add(w);
        if floated {
            to.toggle_float(w);
        }
        true
    }
}
