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

//! Concatenate and ConcatenateResTemplate (sections 19.6.12 and 19.6.13).

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use alloc::vec::Vec;

/// A resource template's EndTag (section 6.4.2.9): 0x79, then a checksum.
const END_TAG: u8 = 0x79;

impl<H: Host> Machine<'_, H> {
    /// The first operand's type decides the result's: two integers make a
    /// buffer of both, a string takes the second as text, a buffer takes it
    /// as bytes.
    pub(crate) fn concat(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let k = aml.kids(t, node);
        let (a, b) = (self.value(aml, t, k[0])?, self.value(aml, t, k[1])?);
        let w = self.width(aml, t);
        let joined = match &a {
            Object::Integer(_) => b.as_integer(w).and_then(|y| {
                let mut v = a.as_bytes(w)?;
                v.extend_from_slice(&y.to_le_bytes()[..w.bytes()]);
                Some(Object::buffer(v))
            }),
            Object::String(_) => {
                b.as_text(w).and_then(|y| Some(Object::string(&[a.as_text(w)?, y].concat())))
            }
            Object::Buffer(_) => {
                b.as_bytes(w).and_then(|y| Some(Object::buffer([a.as_bytes(w)?, y].concat())))
            }
            _ => None,
        };
        let joined = joined.ok_or_else(|| self.fault(FaultKind::Type, at))?;
        self.charge(joined.size() as u64, at)?;
        self.result(aml, t, k[2], joined, at)
    }

    /// Two resource templates made one: each loses its EndTag, and one EndTag
    /// with a zero checksum (which says "not checked") closes the result.
    pub(crate) fn concat_res(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let k = aml.kids(t, node);
        let mut out = Vec::new();
        for kid in &k[..2] {
            let v = self.value(aml, t, *kid)?;
            let Object::Buffer(b) = v else { return Err(self.fault(FaultKind::Type, at)) };
            let b = b.borrow();
            let body = match b.len().checked_sub(2) {
                Some(n) if b[n] == END_TAG => &b[..n],
                _ => &b[..],
            };
            out.extend_from_slice(body);
        }
        out.extend_from_slice(&[END_TAG, 0]);
        self.charge(out.len() as u64, at)?;
        self.result(aml, t, k[2], Object::buffer(out), at)
    }
}
