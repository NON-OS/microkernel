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

//! A NameString in a term: a call where a TermArg or TermObj may stand, a
//! reference everywhere else.

use super::parser::Parser;
use super::{Pos, Scopes};
use crate::arena::Imm;
use crate::error::{Error, ErrorKind};
use crate::grammar::{name_string, Cursor, NamePath};
use crate::op::Op;

/// MethodFlags hold ArgCount in three bits (section 20.2.5.2), so no method
/// takes more than seven, whatever a namespace answers.
const MAX_ARGS: u8 = 7;

impl<S: Scopes> Parser<'_, S> {
    pub(super) fn name_term(&mut self, c: &mut Cursor<'_>, pos: Pos) -> Result<u32, Error> {
        let at = c.pos;
        if pos == Pos::Data {
            return Err(Error::new(ErrorKind::Misplaced(Op::NAME_STRING.0), at));
        }
        let path = name_string(c)?;
        let mark = self.arena.mark();
        // A name in a SuperName is never called, though the grammar lets a
        // MethodInvocation stand there: CondRefOf(\_OSI) names a method of
        // one argument without passing it, and only reading it as a name
        // keeps the Target after it a Target.
        if pos.calls() {
            for _ in 0..self.scopes.arity(self.bytes, path).min(MAX_ARGS) {
                let a = self.term(c, Pos::Term)?;
                self.arena.push_kid(a);
            }
        }
        self.arena.finish(Op::NAME_STRING, at, Imm::Name(path), mark)
    }

    /// A NameString operand that defines or refers, never calls.
    pub(super) fn name_only(&mut self, c: &mut Cursor<'_>) -> Result<(u32, NamePath), Error> {
        let at = c.pos;
        let path = name_string(c)?;
        let mark = self.arena.mark();
        Ok((self.arena.finish(Op::NAME_STRING, at, Imm::Name(path), mark)?, path))
    }
}
