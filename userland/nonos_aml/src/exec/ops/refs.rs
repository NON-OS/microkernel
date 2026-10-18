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

//! RefOf, DerefOf, CondRefOf (sections 19.6.115, 19.6.31 and 19.6.15).

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn refs(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let a = aml.kid(t, node, 0);
        match op {
            Op::REF_OF => {
                let place = self.place(aml, t, a)?;
                Ok(Object::Ref(self.reference(aml, place, at)?))
            }
            Op::DEREF_OF => self.deref_of(aml, t, a, at),
            Op::COND_REF_OF => self.cond_ref_of(aml, t, node, at),
            Op::INDEX => self.index(aml, t, node, at),
            Op::SIZE_OF => self.size_of(aml, t, a, at),
            _ => self.object_type(aml, t, a, at),
        }
    }

    /// A reference reads as what it refers to; a string is a path, resolved
    /// from the current scope.
    fn deref_of(&mut self, aml: &mut Aml, t: u16, a: u32, at: u32) -> Result<Object, Fault> {
        match self.term(aml, t, a)? {
            Object::Ref(r) => self.read_ref(aml, &r, at),
            Object::String(s) => {
                let path = s.borrow().clone();
                let id = self.resolve_text(aml, &path, at)?;
                self.read_named(aml, id, alloc::vec::Vec::new(), at)
            }
            _ => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
