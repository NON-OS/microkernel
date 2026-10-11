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

//! Store and CopyObject as operators: the value goes to the target and is
//! the result (sections 19.6.132 and 19.6.27).

use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    /// The source is taken as it is: a reference stored to a local makes the
    /// local hold that reference, which DerefOf then follows.
    pub(crate) fn store_op(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let (a, b) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        let v = self.term(aml, t, a)?;
        let place = self.place(aml, t, b)?;
        self.write(aml, place, v.clone(), at, op == Op::COPY_OBJECT)?;
        Ok(v)
    }
}
