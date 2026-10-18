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

//! Filling the arena, for the parser alone.

use super::{Arena, Imm, Node, Span};
use crate::error::{Error, ErrorKind};
use crate::op::Op;

impl Arena {
    pub(crate) fn mark(&self) -> usize {
        self.stack.len()
    }

    pub(crate) fn push_kid(&mut self, i: u32) {
        self.stack.push(i);
    }

    /// Makes a node of everything pushed since `mark`.
    pub(crate) fn finish(&mut self, op: Op, at: u32, imm: Imm, mark: usize) -> Result<u32, Error> {
        if self.nodes.len() >= self.limits.max_nodes {
            return Err(Error::new(ErrorKind::Nodes, at));
        }
        let kids = Span { at: self.kids.len() as u32, len: (self.stack.len() - mark) as u32 };
        self.kids.extend(self.stack.drain(mark..));
        self.nodes.push(Node { op, at, imm, kids });
        Ok(self.nodes.len() as u32 - 1)
    }

    /// Where the arena stands, to return to if a later parse fails.
    pub(crate) fn checkpoint(&self) -> (usize, usize) {
        (self.nodes.len(), self.kids.len())
    }

    /// Drops everything a failed parse left, so the arena holds only whole
    /// objects.
    pub(crate) fn rollback(&mut self, to: (usize, usize)) {
        self.nodes.truncate(to.0);
        self.kids.truncate(to.1);
        self.stack.clear();
    }
}
