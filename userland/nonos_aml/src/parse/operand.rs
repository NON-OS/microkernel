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

//! One operand of an object, by its kind in the shape row.

use super::parser::Parser;
use super::{Pos, Scopes};
use crate::arena::{Imm, Span};
use crate::error::Error;
use crate::grammar::{Cursor, NamePath};
use crate::op::Op;
use crate::shape::Arg;

impl<S: Scopes> Parser<'_, S> {
    /// `name` collects the object's first NameString, the one a Scope operand
    /// opens; `imms` the first two fixed-width operands (unset slots hold
    /// u64::MAX, which no fixed-width operand can), which Method and External
    /// declare arities from.
    pub(super) fn operand(
        &mut self,
        c: &mut Cursor<'_>,
        arg: Arg,
        name: &mut Option<NamePath>,
        imms: &mut [u64; 2],
    ) -> Result<u32, Error> {
        let (at, mark) = (c.pos, self.arena.mark());
        let (op, imm) = match arg {
            Arg::Term => return self.term(c, Pos::Term),
            Arg::Super => return self.term(c, Pos::Super),
            Arg::Simple => return self.term(c, Pos::Simple),
            Arg::Data => return self.term(c, Pos::Data),
            Arg::Terms => return self.terms(c),
            Arg::Fields => return self.fields(c),
            Arg::Elements => return self.elements(c),
            Arg::Scope => return self.scoped(c, *name),
            Arg::Name => {
                let (node, path) = self.name_only(c)?;
                name.get_or_insert(path);
                return Ok(node);
            }
            Arg::U8 | Arg::U16 | Arg::U32 => {
                let v = c.le(match arg {
                    Arg::U8 => 1,
                    Arg::U16 => 2,
                    _ => 4,
                })?;
                if let Some(slot) = imms.iter_mut().find(|s| **s == u64::MAX) {
                    *slot = v;
                }
                (Op::IMM, Imm::Int(v))
            }
            Arg::Bytes | Arg::Body => {
                let len = c.end - c.pos;
                c.take(len)?;
                let op = if arg == Arg::Body { Op::DEFERRED } else { Op::BYTES };
                (op, Imm::Bytes(Span { at, len }))
            }
        };
        self.arena.finish(op, at, imm, mark)
    }
}
