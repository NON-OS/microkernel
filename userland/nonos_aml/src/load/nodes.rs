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

//! Reading a loaded table's nodes by value, so no borrow of the arena is held
//! while the namespace or the interpreter changes.

use super::Aml;
use crate::arena::{Imm, Node};
use crate::grammar::NamePath;
use crate::op::Op;
use alloc::vec::Vec;

impl Aml {
    pub(crate) fn node(&self, table: u16, node: u32) -> Node {
        // Every index handed here came from the same arena's kid lists.
        *self.tables[table as usize].arena.node(node).expect("node of this arena")
    }

    pub(crate) fn kids(&self, table: u16, node: u32) -> Vec<u32> {
        self.tables[table as usize].arena.kids(node).to_vec()
    }

    pub(crate) fn kid(&self, table: u16, node: u32, i: usize) -> u32 {
        self.tables[table as usize].arena.kids(node)[i]
    }

    pub(crate) fn kid_count(&self, table: u16, node: u32) -> usize {
        self.tables[table as usize].arena.kids(node).len()
    }

    /// The NameString a NAME_STRING node carries.
    pub(crate) fn name_of(&self, table: u16, node: u32) -> Option<NamePath> {
        match self.node(table, node) {
            Node { op: Op::NAME_STRING, imm: Imm::Name(p), .. } => Some(p),
            _ => None,
        }
    }

    /// The integer an IMM or constant node carries.
    pub(crate) fn imm(&self, table: u16, node: u32) -> u64 {
        match self.node(table, node).imm {
            Imm::Int(v) => v,
            _ => 0,
        }
    }

    /// The bytes a string, BYTES or DEFERRED node spans.
    pub(crate) fn span_bytes(&self, table: u16, node: u32) -> &[u8] {
        match self.node(table, node).imm {
            Imm::Bytes(s) => &self.tables[table as usize].bytes[s.at as usize..s.end() as usize],
            _ => &[],
        }
    }
}
