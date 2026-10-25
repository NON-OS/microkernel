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

//! PackageElementList, section 20.2.5.4: data objects and names, to the end
//! of the package.

use super::parser::Parser;
use super::{Pos, Scopes};
use crate::arena::Imm;
use crate::error::Error;
use crate::grammar::Cursor;
use crate::op::Op;

impl<S: Scopes> Parser<'_, S> {
    /// NumElements is not checked against the count here: the grammar lets a
    /// package declare more elements than it initializes, and what to do with
    /// fewer is the interpreter's rule, not the byte grammar's.
    pub(super) fn elements(&mut self, c: &mut Cursor<'_>) -> Result<u32, Error> {
        let (at, mark) = (c.pos, self.arena.mark());
        while !c.done() {
            let e = self.term(c, Pos::Element)?;
            self.arena.push_kid(e);
        }
        self.arena.finish(Op::ELEMENTS, at, Imm::None, mark)
    }
}
