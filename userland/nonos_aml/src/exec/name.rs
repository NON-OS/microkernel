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

//! A NameString in a term: a method called, or an object read.

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::Object;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// The object a NAME_STRING node names from the current scope.
    pub(crate) fn lookup(&self, aml: &Aml, t: u16, node: u32) -> Result<Id, Fault> {
        let at = aml.node(t, node).at;
        let path = aml.name_of(t, node).ok_or_else(|| self.fault(FaultKind::Type, at))?;
        let bytes = &aml.tables[t as usize].bytes;
        aml.ns
            .resolve(self.frame().scope, bytes, path)
            .ok_or_else(|| self.fault(FaultKind::Undefined, at))
    }

    pub(crate) fn name_term(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let id = self.lookup(aml, t, node)?;
        let mut args = Vec::new();
        for i in 0..aml.kid_count(t, node) {
            // An argument is passed as it stands: RefOf(Local0) arrives as the
            // reference, which is how a method writes back to its caller.
            args.push(self.term(aml, t, aml.kid(t, node, i))?);
        }
        self.read_named(aml, id, args, aml.node(t, node).at)
    }

    /// What reading a named object gives: a method's result, a Name's value,
    /// or the object itself for one that holds no data.
    pub(crate) fn read_named(
        &mut self,
        aml: &mut Aml,
        id: Id,
        args: Vec<Object>,
        at: u32,
    ) -> Result<Object, Fault> {
        let e = *aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        match e.kind {
            Kind::Method(_) => self.call(aml, id, args, at),
            Kind::Name => self.name_value(aml, id, at),
            // Field units and buffer fields read through their region or
            // buffer, which arrives with unit F 4.
            Kind::FieldUnit(_) | Kind::BufferField => Err(self.fault(FaultKind::Unsupported, at)),
            _ => Ok(Object::Named(id, e.gen)),
        }
    }
}
