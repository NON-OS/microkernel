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

//! Expressions, by family.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::{Family, Op};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn expr(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
        at: u32,
    ) -> Result<Object, Fault> {
        match op.family() {
            Family::Binary => self.binary(aml, t, node, op),
            Family::Divide => self.divide(aml, t, node),
            Family::Unary => self.unary(aml, t, node, op),
            Family::Step => self.step(aml, t, node, op),
            Family::Logic => self.logic(aml, t, node, op),
            Family::Text => self.text(aml, t, node, op),
            Family::Store => self.store_op(aml, t, node, op),
            Family::Refs => self.refs(aml, t, node, op),
            Family::Match => self.match_op(aml, t, node),
            Family::Sync => self.sync_expr(aml, t, node, op),
            Family::Timer => Ok(Object::Integer(self.host.timer())),
            Family::Load => self.load_op(aml, t, node, op, at),
            Family::None => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
