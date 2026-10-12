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

//! Unload (section 19.6.146) as a statement.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// The operand is a DDBHandle Load or LoadTable gave; unloading the
    /// DSDT, or anything else, is a fault.
    pub(crate) fn unload_stmt(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        at: u32,
    ) -> Result<(), Fault> {
        let k = aml.kid(t, node, 0);
        match self.value(aml, t, k)? {
            Object::Table(i) if aml.unload(i) => Ok(()),
            _ => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
