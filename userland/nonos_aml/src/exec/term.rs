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

//! Evaluating a TermArg.

use super::{FaultKind, Host, Machine};
use crate::arena::Imm;
use crate::load::Aml;
use crate::object::Object;
use crate::op::{Family, Op};

/// What Revision returns: this interpreter's own revision, not the
/// specification's.
pub const REVISION: u64 = 1;

impl<H: Host> Machine<'_, H> {
    /// A term's value as it stands: a reference stays a reference.
    pub(crate) fn term(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
    ) -> Result<Object, super::Fault> {
        let n = aml.node(t, node);
        self.tick(n.at)?;
        let w = self.width(aml, t);
        match n.op {
            op if op.is_integer_constant() => match n.imm {
                Imm::Int(v) => Ok(Object::Integer(w.mask(v))),
                _ => Err(self.fault(FaultKind::Type, n.at)),
            },
            Op::STRING => {
                let len = aml.span_bytes(t, node).len() as u64;
                self.charge(len, n.at)?;
                Ok(Object::string(aml.span_bytes(t, node)))
            }
            Op::REVISION => Ok(Object::Integer(REVISION)),
            Op::BUFFER => self.buffer(aml, t, node),
            Op::PACKAGE | Op::VAR_PACKAGE => self.package(aml, t, node),
            Op::NAME_STRING => self.name_term(aml, t, node),
            op if op.local().is_some() || op.arg().is_some() => self.read_var(op, n.at),
            op if op.family() != Family::None => self.expr(aml, t, node, op, n.at),
            _ => Err(self.fault(FaultKind::Type, n.at)),
        }
    }
}
