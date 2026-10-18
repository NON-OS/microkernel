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

//! Buffer and Package (section 19.6.10 and 19.6.101) as they are evaluated.

use super::{Fault, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::Op;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// BufferSize, then the initializer; a buffer is never shorter than its
    /// initializer, and the rest is zero.
    pub(crate) fn buffer(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let size = self.int(aml, t, aml.kid(t, node, 0))?;
        let init = aml.span_bytes(t, aml.kid(t, node, 1)).to_vec();
        let len = size.max(init.len() as u64);
        self.charge(len, at)?;
        let mut bytes = init;
        bytes.resize(len as usize, 0);
        Ok(Object::buffer(bytes))
    }

    /// NumElements (or VarNumElements), then the initializers. A package
    /// holds as many elements as it declares, or as it initializes if that
    /// is more (unverified: the text's rule for too many initializers is not
    /// to hand); the rest are uninitialized. A name in a package is a
    /// reference to the object it names, or its own text if it names none.
    pub(crate) fn package(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let n = aml.node(t, node);
        let count_node = aml.kid(t, node, 0);
        let count = match n.op {
            Op::VAR_PACKAGE => self.int(aml, t, count_node)?,
            _ => aml.imm(t, count_node),
        };
        let inits = aml.kids(t, aml.kid(t, node, 1));
        let len = count.max(inits.len() as u64);
        self.charge(len.saturating_mul(core::mem::size_of::<Object>() as u64), n.at)?;
        let mut elems = Vec::with_capacity(len as usize);
        for e in inits {
            let v = match aml.node(t, e).op {
                Op::NAME_STRING => self.element_name(aml, t, e),
                _ => self.term(aml, t, e)?,
            };
            elems.push(v);
        }
        elems.resize(len as usize, Object::Uninit);
        Ok(Object::package(elems))
    }
}
