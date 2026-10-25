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

//! Writing through a reference, and the copies Store hands on.

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{Object, Ref};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn write_ref(
        &mut self,
        aml: &mut Aml,
        r: Ref,
        v: Object,
        at: u32,
        copy: bool,
    ) -> Result<(), Fault> {
        match r {
            Ref::Named(id, gen) if aml.ns.live(id, gen) => self.write_named(aml, id, v, at, copy),
            Ref::Named(..) => Err(self.fault(FaultKind::Stale, at)),
            Ref::Slot(s) => {
                *s.borrow_mut() = self.copy(&v, at)?;
                Ok(())
            }
            Ref::Element(p, i) => {
                let v = self.copy(&v, at)?;
                let mut p = p.borrow_mut();
                let e = p.get_mut(i).ok_or_else(|| self.fault(FaultKind::Index, at))?;
                *e = v;
                Ok(())
            }
            Ref::Byte(b, i) => {
                let x = self.deref(aml, v, at)?.as_integer(crate::object::Width::WIDE);
                let x = x.ok_or_else(|| self.fault(FaultKind::Type, at))?;
                let mut b = b.borrow_mut();
                let e = b.get_mut(i).ok_or_else(|| self.fault(FaultKind::Index, at))?;
                *e = x as u8;
                Ok(())
            }
        }
    }

    /// A copy of `v` that shares nothing with it, counted against the budget.
    pub(crate) fn copy(&mut self, v: &Object, at: u32) -> Result<Object, Fault> {
        self.charge(v.size() as u64, at)?;
        v.deep_copy().ok_or_else(|| self.fault(FaultKind::Alloc, at))
    }
}
