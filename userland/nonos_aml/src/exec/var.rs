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

//! Locals and arguments.

use super::{Fault, FaultKind, Host, Machine, Slot};
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn slot(&self, op: Op) -> Option<Slot> {
        let f = self.frame();
        match (op.local(), op.arg()) {
            (Some(i), _) => Some(f.locals[i as usize].clone()),
            (_, Some(i)) => Some(f.args[i as usize].clone()),
            _ => None,
        }
    }

    /// Reading a local or argument nothing was stored to is an error
    /// (section 19.3.5); an argument holding a reference reads as that
    /// reference, for the caller to follow.
    pub(crate) fn read_var(&mut self, op: Op, at: u32) -> Result<Object, Fault> {
        let slot = self.slot(op).ok_or_else(|| self.fault(FaultKind::Type, at))?;
        let v = slot.borrow().clone();
        match v {
            Object::Uninit => Err(self.fault(FaultKind::Uninit, at)),
            v => Ok(v),
        }
    }
}
