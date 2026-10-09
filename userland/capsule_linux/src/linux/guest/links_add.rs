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

use crate::linux::abi::errno;

use super::links::Links;
use super::links_room::room;

impl Links {
    /// A new link at `path`: EEXIST when `path` already is one, ENOSPC once
    /// the table is full (`links_room`).
    pub fn add(&self, path: Vec<u8>, target: Vec<u8>) -> Result<(), i64> {
        if self.target(&path).is_some() {
            return Err(errno::EEXIST);
        }
        if !room(self.0.borrow().len()) {
            return Err(errno::ENOSPC);
        }
        self.0.borrow_mut().push((path, target));
        Ok(())
    }
}
