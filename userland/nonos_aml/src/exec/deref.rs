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

//! Following a reference to the object it refers to.

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{Object, Ref};
use alloc::vec::Vec;

/// A reference may refer to a slot holding another reference; a chain this
/// long is a loop a table built, not data.
const MAX_HOPS: u32 = 16;

impl<H: Host> Machine<'_, H> {
    /// `v` if it is not a reference, else what it refers to, followed until
    /// it is not.
    pub(crate) fn deref(&mut self, aml: &mut Aml, mut v: Object, at: u32) -> Result<Object, Fault> {
        for _ in 0..MAX_HOPS {
            let Object::Ref(r) = v else { return Ok(v) };
            v = self.read_ref(aml, &r, at)?;
        }
        Err(self.fault(FaultKind::Type, at))
    }

    /// One step: what a reference refers to now.
    pub(crate) fn read_ref(&mut self, aml: &mut Aml, r: &Ref, at: u32) -> Result<Object, Fault> {
        match r {
            Ref::Named(id, gen) if aml.ns.live(*id, *gen) => {
                self.read_named(aml, *id, Vec::new(), at)
            }
            Ref::Named(..) => Err(self.fault(FaultKind::Stale, at)),
            Ref::Slot(s) => Ok(s.borrow().clone()),
            Ref::Element(p, i) => {
                let p = p.borrow();
                p.get(*i).cloned().ok_or_else(|| self.fault(FaultKind::Index, at))
            }
            Ref::Byte(b, i) => {
                let b = b.borrow();
                b.get(*i)
                    .map(|x| Object::Integer(*x as u64))
                    .ok_or_else(|| self.fault(FaultKind::Index, at))
            }
        }
    }
}
