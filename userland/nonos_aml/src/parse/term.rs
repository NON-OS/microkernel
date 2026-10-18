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

//! One term, by where it stands.

use super::parser::Parser;
use super::{Pos, Scopes};
use crate::arena::Imm;
use crate::error::{Error, ErrorKind};
use crate::grammar::{is_name_start, Cursor};
use crate::op::Op;
use crate::shape::lookup;

impl<S: Scopes> Parser<'_, S> {
    /// A TermList, to the cursor's end.
    pub(super) fn terms(&mut self, c: &mut Cursor<'_>) -> Result<u32, Error> {
        let (at, mark) = (c.pos, self.arena.mark());
        while !c.done() {
            let t = self.term(c, Pos::Statement)?;
            self.arena.push_kid(t);
        }
        self.arena.finish(Op::TERMS, at, Imm::None, mark)
    }

    /// Every nesting goes through here, so this is where depth is bounded.
    pub(super) fn term(&mut self, c: &mut Cursor<'_>, pos: Pos) -> Result<u32, Error> {
        if self.depth >= self.arena.limits.max_depth {
            return Err(Error::new(ErrorKind::Depth, c.pos));
        }
        self.depth += 1;
        let r = self.term_at(c, pos);
        self.depth -= 1;
        r
    }

    fn term_at(&mut self, c: &mut Cursor<'_>, pos: Pos) -> Result<u32, Error> {
        let at = c.pos;
        let b = c.peek().ok_or(Error::new(ErrorKind::Truncated, at))?;
        // 0x00 is NullName where a name is expected and ZeroOp elsewhere.
        if is_name_start(b) || (b == 0 && matches!(pos, Pos::Super | Pos::Simple)) {
            return self.name_term(c, pos);
        }
        let op = match c.u8()? {
            Op::EXT_PREFIX => Op::ext(c.u8()?),
            b => Op(b as u16),
        };
        if let Some(leaf) = self.leaf(c, op, at, pos)? {
            return Ok(leaf);
        }
        let group = lookup(op).ok_or(Error::new(ErrorKind::Opcode(op.0), at))?;
        if !pos.admits(group.class, op) {
            return Err(Error::new(ErrorKind::Misplaced(op.0), at));
        }
        self.object(c, op, at, group)
    }
}
