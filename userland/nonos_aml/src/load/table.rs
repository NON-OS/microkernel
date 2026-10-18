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

//! A loaded definition block: its bytes, owned, and the arena read from them.

use crate::arena::Arena;
use crate::block::Block;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Table {
    pub bytes: Vec<u8>,
    pub arena: Arena,
    /// The TERMS node of the table's own term list.
    pub root: u32,
}

impl Table {
    /// The block, already checked when the table was loaded.
    pub fn block(&self) -> Block<'_> {
        Block { bytes: &self.bytes }
    }

    /// Section 5.2.11.1: integers are 64 bits wide from revision 2 on.
    pub fn wide(&self) -> bool {
        self.block().wide_integers()
    }
}
