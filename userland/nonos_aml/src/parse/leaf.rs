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

//! The terms with no operands: constants, strings, Revision, locals,
//! arguments and the debug object (sections 20.2.3 and 20.2.6).

use super::parser::Parser;
use super::{Pos, Scopes};
use crate::arena::Imm;
use crate::error::{Error, ErrorKind};
use crate::grammar::Cursor;
use crate::op::Op;

impl<S: Scopes> Parser<'_, S> {
    /// The leaf node for `op`, or None if `op` has operands of its own.
    pub(super) fn leaf(
        &mut self,
        c: &mut Cursor<'_>,
        op: Op,
        at: u32,
        pos: Pos,
    ) -> Result<Option<u32>, Error> {
        // Ones is all bits set at the width the table's revision gives; the
        // interpreter narrows it, so the arena keeps the widest value.
        let (imm, ok) = match op {
            Op::ZERO => (Imm::Int(0), pos.admits_constant()),
            Op::ONE => (Imm::Int(1), pos.admits_constant()),
            Op::ONES => (Imm::Int(u64::MAX), pos.admits_constant()),
            Op::BYTE_CONST => (Imm::Int(c.le(1)?), pos.admits_constant()),
            Op::WORD_CONST => (Imm::Int(c.le(2)?), pos.admits_constant()),
            Op::DWORD_CONST => (Imm::Int(c.le(4)?), pos.admits_constant()),
            Op::QWORD_CONST => (Imm::Int(c.le(8)?), pos.admits_constant()),
            Op::STRING => (Imm::Bytes(self.string(c)?), pos.admits_constant()),
            Op::REVISION => (Imm::None, pos.admits_constant()),
            Op::DEBUG => (Imm::None, pos == Pos::Super),
            _ if op.local().is_some() || op.arg().is_some() => (Imm::None, pos.admits_variable()),
            _ => return Ok(None),
        };
        if !ok {
            return Err(Error::new(ErrorKind::Misplaced(op.0), at));
        }
        let mark = self.arena.mark();
        self.arena.finish(op, at, imm, mark).map(Some)
    }
}
