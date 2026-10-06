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

use super::history::History;

/* Stepping through session history. */
impl History {
    pub fn can_back(&self) -> bool {
        !self.entries.is_empty() && self.index > 0
    }

    pub fn can_forward(&self) -> bool {
        self.index + 1 < self.entries.len()
    }

    /* `delta` entries back (negative) or forward: -1 and 1 for the
     * toolbar's buttons, any step for a page's history.go(). A step past
     * either end moves nothing, as in every browser. */
    pub fn go(&mut self, delta: i32) -> Option<&str> {
        let to = self.index as i64 + delta as i64;
        if delta == 0 || self.entries.is_empty() || to < 0 || to >= self.entries.len() as i64 {
            return None;
        }
        self.index = to as usize;
        self.current()
    }
}
