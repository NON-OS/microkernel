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

//! A term as an operand: references followed, integers converted.

use super::{FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// A term used as an operand: a reference reads as what it refers to.
    pub(crate) fn value(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
    ) -> Result<Object, super::Fault> {
        let v = self.term(aml, t, node)?;
        let at = aml.node(t, node).at;
        self.deref(aml, v, at)
    }

    /// A term converted to an integer at the table's width.
    pub(crate) fn int(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<u64, super::Fault> {
        let v = self.value(aml, t, node)?;
        let w = self.width(aml, t);
        v.as_integer(w).ok_or_else(|| self.fault(FaultKind::Type, aml.node(t, node).at))
    }
}
