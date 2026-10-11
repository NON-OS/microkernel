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

//! A named object defined while code runs: inside a method, where it lasts
//! until the method returns, or in load-time code.

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    /// The loader makes the object, as it would for a table, in the current
    /// scope; the method's journal records it for removal. A Name made this
    /// way takes its value now, while the locals it may be built from hold
    /// what they hold at this point.
    pub(crate) fn define_here(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<(), Fault> {
        let scope = self.frame().scope;
        let before = (aml.notes.list.len(), aml.notes.dropped);
        aml.define(t, node, scope, self);
        if (aml.notes.list.len(), aml.notes.dropped) != before {
            // A definition a method makes and cannot complete is a fault in
            // that method, not a note about a table.
            let at = aml.notes.list.pop().map_or(aml.node(t, node).at, |n| n.at);
            return Err(self.fault(FaultKind::Undefined, at));
        }
        if aml.node(t, node).op != Op::NAME {
            return Ok(());
        }
        let name = aml.kid(t, node, 0);
        let id = self.lookup(aml, t, name)?;
        let value = aml.kid(t, node, 1);
        let v = self.term(aml, t, value)?;
        *aml.rt.value(id) = Some(v);
        Ok(())
    }
}
