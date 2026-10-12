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

//! Calling a method (section 19.6.82).

use super::{Fault, FaultKind, Flow, Frame, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin, Predefined};
use crate::object::Object;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// Arguments are copied in, so the method cannot change the caller's
    /// objects except through a reference it was given. What the method
    /// defines is removed when it returns, however it returns.
    pub(crate) fn call(
        &mut self,
        aml: &mut Aml,
        id: Id,
        args: Vec<Object>,
        at: u32,
    ) -> Result<Object, Fault> {
        let e = *aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        let Kind::Method(info) = e.kind else { return Err(self.fault(FaultKind::Type, at)) };
        let (table, node) = match e.origin {
            Origin::Table { table, node } => (table, node),
            Origin::Predefined(Predefined::Osi) => return self.osi(aml, &args, at),
            Origin::Predefined(_) => return Err(self.fault(FaultKind::Type, at)),
        };
        let deep = super::stack::used_since(self.stack_top) > self.limits.stack_bytes;
        if self.calls >= self.limits.call_depth || deep {
            return Err(self.fault(FaultKind::CallDepth, at));
        }
        let body = self.body(aml, table, node, id)?;
        let mut frame = Frame::new(table, id);
        for (slot, a) in frame.args.iter().zip(args) {
            *slot.borrow_mut() = a.deep_copy().ok_or_else(|| self.fault(FaultKind::Alloc, at))?;
        }
        if info.serialized() {
            self.acquire_mutex(aml, id, info.sync_level(), 0xFFFF, at)?;
            frame.serial = Some(id);
        }
        frame.journal = Some(aml.ns.journal_begin());
        self.frames.push(frame);
        self.calls += 1;
        let flow = self.terms(aml, table, body);
        self.calls -= 1;
        let frame = self.frames.pop().expect("the frame pushed above");
        let made = aml.ns.journal_unwind(frame.journal.unwrap_or(0));
        for gone in made {
            aml.rt.forget(gone);
        }
        if let Some(m) = frame.serial {
            self.release_mutex(aml, m, at)?;
        }
        match flow? {
            Flow::Return(v) => Ok(v),
            Flow::Normal => Ok(Object::Uninit),
            Flow::Break | Flow::Continue => Err(self.fault(FaultKind::Control, at)),
        }
    }
}
