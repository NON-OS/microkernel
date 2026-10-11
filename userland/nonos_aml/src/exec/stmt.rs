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

//! A TermObj in a TermList: a statement, a definition, or an expression whose
//! value is dropped.

use super::{Fault, FaultKind, Flow, Host, Machine};
use crate::load::Aml;
use crate::op::Op;
use crate::shape::{lookup, Class};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn stmt(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Flow, Fault> {
        let n = aml.node(t, node);
        self.tick(n.at)?;
        match n.op {
            Op::IF => return self.if_(aml, t, node),
            Op::WHILE => return self.while_(aml, t, node),
            Op::BREAK => return Ok(Flow::Break),
            Op::CONTINUE => return Ok(Flow::Continue),
            Op::RETURN => {
                let k = aml.kid(t, node, 0);
                return Ok(Flow::Return(self.value(aml, t, k)?));
            }
            Op::NOOP | Op::BREAK_POINT => {}
            Op::NOTIFY => self.notify(aml, t, node)?,
            Op::RELEASE | Op::RESET | Op::SIGNAL => self.sync_stmt(aml, t, node, n.op)?,
            Op::SLEEP | Op::STALL => self.wait_stmt(aml, t, node, n.op)?,
            Op::FATAL => return Err(self.fatal(aml, t, node)),
            // Unload removes a table loaded by Load, which arrives with
            // region access (unit F 4).
            Op::UNLOAD => return Err(self.fault(FaultKind::Unsupported, n.at)),
            op if lookup(op).is_some_and(|g| g.class == Class::Object) => {
                self.define_here(aml, t, node)?
            }
            _ => {
                self.term(aml, t, node)?;
            }
        }
        Ok(Flow::Normal)
    }

    /// Fatal (section 19.6.48): reported, then the evaluation stops.
    fn fatal(&mut self, aml: &mut Aml, t: u16, node: u32) -> Fault {
        let at = aml.node(t, node).at;
        let (kind, code) = (aml.imm(t, aml.kid(t, node, 0)), aml.imm(t, aml.kid(t, node, 1)));
        let argn = aml.kid(t, node, 2);
        let arg = self.int(aml, t, argn).unwrap_or(0);
        self.host.fatal(kind as u8, code as u32, arg);
        self.fault(FaultKind::Fatal, at)
    }
}
