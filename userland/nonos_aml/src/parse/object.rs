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

//! An object read by its shape row.

use super::parser::Parser;
use super::Scopes;
use crate::arena::Imm;
use crate::error::{Error, ErrorKind};
use crate::grammar::{pkg_end, Cursor, NamePath};
use crate::op::Op;
use crate::shape::{lookup, Group};

/// ObjectType's value for a method in External, section 20.2.5.2.
const EXTERNAL_METHOD: u64 = 8;

impl<S: Scopes> Parser<'_, S> {
    pub(super) fn object(
        &mut self,
        c: &mut Cursor<'_>,
        op: Op,
        at: u32,
        group: &Group,
    ) -> Result<u32, Error> {
        let mark = self.arena.mark();
        let mut inner = *c;
        if group.pkg {
            let end = pkg_end(c)?;
            inner = c.inner(end)?;
        }
        let mut name: Option<NamePath> = None;
        let mut imms = [u64::MAX; 2];
        for arg in group.args {
            let k = self.operand(&mut inner, *arg, &mut name, &mut imms)?;
            self.arena.push_kid(k);
        }
        // Every row with a package ends in an operand that reads to its end.
        c.pos = if group.pkg { inner.end } else { inner.pos };
        if op == Op::IF && c.peek() == Some(Op::ELSE.0 as u8) {
            let else_at = c.take(1)?;
            let row = lookup(Op::ELSE).ok_or(Error::new(ErrorKind::Opcode(Op::ELSE.0), else_at))?;
            let e = self.object(c, Op::ELSE, else_at, row)?;
            self.arena.push_kid(e);
        }
        if let Some(name) = name {
            // ArgCount is MethodFlags bits 0-2.
            if op == Op::METHOD {
                self.scopes.declare(self.bytes, name, (imms[0] & 7) as u8);
            } else if op == Op::EXTERNAL && imms[0] == EXTERNAL_METHOD {
                self.scopes.declare(self.bytes, name, (imms[1] & 7) as u8);
            }
        }
        self.arena.finish(op, at, Imm::None, mark)
    }
}
