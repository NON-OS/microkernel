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

//! Index (section 19.6.62).

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{Object, Ref};

impl<H: Host> Machine<'_, H> {
    /// Index's source made into the reference it returns (section 19.6.62).
    pub(crate) fn index(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        at: u32,
    ) -> Result<Object, Fault> {
        let k = aml.kids(t, node);
        let src = self.value(aml, t, k[0])?;
        let i = usize::try_from(self.int(aml, t, k[1])?).unwrap_or(usize::MAX);
        let r = match src {
            Object::Package(p) if i < p.borrow().len() => Ref::Element(p, i),
            Object::Buffer(b) | Object::String(b) if i < b.borrow().len() => Ref::Byte(b, i),
            Object::Package(_) | Object::Buffer(_) | Object::String(_) => {
                return Err(self.fault(FaultKind::Index, at))
            }
            _ => return Err(self.fault(FaultKind::Type, at)),
        };
        self.result(aml, t, k[2], Object::Ref(r), at)
    }
}
