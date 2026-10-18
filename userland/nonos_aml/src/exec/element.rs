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

//! A name in a package: a reference to what it names, or its text.

use super::{Host, Machine};
use crate::load::Aml;
use crate::object::{Object, Ref};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn element_name(&mut self, aml: &mut Aml, t: u16, e: u32) -> Object {
        let scope = self.frame().scope;
        let found =
            aml.name_of(t, e).and_then(|p| aml.ns.resolve(scope, &aml.tables[t as usize].bytes, p));
        match found.and_then(|id| Some((id, aml.ns.get(id)?.gen))) {
            Some((id, gen)) => Object::Ref(Ref::Named(id, gen)),
            None => Object::string(&self.path_text(aml, t, e)),
        }
    }
}
