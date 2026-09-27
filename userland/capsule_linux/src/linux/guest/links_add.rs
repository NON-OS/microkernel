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

//! A link made at run time, by `symlink`, joining the family's table.

use alloc::vec::Vec;

use super::links::Links;

impl Links {
    /// A new link at `path`; false when `path` already is one.
    pub fn add(&self, path: Vec<u8>, target: Vec<u8>) -> bool {
        if self.target(&path).is_some() {
            return false;
        }
        self.0.borrow_mut().push((path, target));
        true
    }
}
