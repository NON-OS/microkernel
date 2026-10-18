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

//! A field's connection (section 19.6.18, Connection): the resource template
//! naming the GPIO pins or serial bus device its accesses go to.

use super::index_bank::Ctx;
use crate::exec::{Fault, FaultKind, Frame, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::Op;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// The template's bytes: a Buffer written in the field list, or the
    /// Name holding one.
    pub(crate) fn connection(
        &mut self,
        aml: &mut Aml,
        ctx: &Ctx,
        at: u32,
    ) -> Result<Option<Vec<u8>>, Fault> {
        let Some(node) = ctx.unit.connection else { return Ok(None) };
        let k = aml.kid(ctx.table, node, 0);
        self.frames.push(Frame::new(ctx.table, ctx.scope));
        let v = match aml.node(ctx.table, k).op {
            Op::NAME_STRING => self
                .lookup(aml, ctx.table, k)
                .and_then(|id| self.read_named(aml, id, Vec::new(), at)),
            _ => self.term(aml, ctx.table, k),
        };
        self.frames.pop();
        match self.deref(aml, v?, at)? {
            Object::Buffer(b) => Ok(Some(b.borrow().clone())),
            _ => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
