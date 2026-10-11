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

//! Where a SuperName or Target points.

use super::{Fault, FaultKind, Host, Machine, Slot};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::{Object, Ref};
use crate::op::Op;

pub enum Place {
    /// NullName as a Target: the result is not stored.
    Null,
    Debug,
    Local(Slot),
    Arg(Slot),
    Named(Id),
    Ref(Ref),
}

impl<H: Host> Machine<'_, H> {
    pub(crate) fn place(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Place, Fault> {
        let n = aml.node(t, node);
        match n.op {
            Op::NAME_STRING if aml.name_of(t, node).is_some_and(|p| p.is_null()) => Ok(Place::Null),
            Op::NAME_STRING => Ok(Place::Named(self.lookup(aml, t, node)?)),
            Op::DEBUG => Ok(Place::Debug),
            op if op.local().is_some() => Ok(Place::Local(self.slot(op).expect("a local"))),
            op if op.arg().is_some() => Ok(Place::Arg(self.slot(op).expect("an argument"))),
            // RefOf, DerefOf and Index name a place through the reference
            // they evaluate to.
            _ => match self.term(aml, t, node)? {
                Object::Ref(r) => Ok(Place::Ref(r)),
                _ => Err(self.fault(FaultKind::Type, n.at)),
            },
        }
    }

    /// The reference RefOf makes to a place.
    pub(crate) fn reference(&self, aml: &Aml, place: Place, at: u32) -> Result<Ref, Fault> {
        match place {
            Place::Named(id) => {
                let gen = aml
                    .ns
                    .get(id)
                    .map(|e| e.gen)
                    .ok_or_else(|| self.fault(FaultKind::Stale, at))?;
                Ok(Ref::Named(id, gen))
            }
            Place::Local(s) | Place::Arg(s) => Ok(Ref::Slot(s)),
            Place::Ref(r) => Ok(r),
            Place::Null | Place::Debug => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
