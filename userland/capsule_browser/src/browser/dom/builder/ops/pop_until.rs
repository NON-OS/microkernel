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

use super::state::Builder;

impl Builder {
    /// Pop until an HTML element named `name` has been popped.
    pub(in super::super) fn pop_until(&mut self, name: &str) {
        while let Some(id) = self.pop() {
            if self.is(id, name) {
                return;
            }
        }
    }

    /// Pop until an HTML element with one of `names` has been popped.
    pub(in super::super) fn pop_until_any(&mut self, names: &[&str]) {
        while let Some(id) = self.pop() {
            if self.is_any(id, names) {
                return;
            }
        }
    }

    /// Pop until only `len` elements are open.
    pub(in super::super) fn open_truncate(&mut self, len: usize) {
        while self.open.len() > len {
            self.pop();
        }
    }
}
