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

//! The conversion operators: ToBuffer, ToDecimalString, ToHexString,
//! ToInteger, ToString (sections 19.6.138 to 19.6.143), and Concatenate,
//! ConcatenateResTemplate and Mid by way of their own files.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{buffer_to_string, Object};
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn text(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        match op {
            Op::CONCAT => return self.concat(aml, t, node),
            Op::CONCAT_RES => return self.concat_res(aml, t, node),
            Op::MID => return self.mid(aml, t, node),
            _ => {}
        }
        let at = aml.node(t, node).at;
        let k = aml.kids(t, node);
        let v = self.value(aml, t, k[0])?;
        let w = self.width(aml, t);
        let made = match op {
            Op::TO_BUFFER => v.to_buffer_explicit(w).map(Object::buffer),
            Op::TO_DECIMAL_STRING => v.to_decimal_string(w).map(|s| Object::string(&s)),
            Op::TO_HEX_STRING => v.to_hex_string(w).map(|s| Object::string(&s)),
            Op::TO_INTEGER => v.to_integer_explicit(w).map(Object::Integer),
            _ => {
                let len = self.int(aml, t, k[1])?;
                v.as_bytes(w).map(|b| Object::string(&buffer_to_string(&b, len)))
            }
        };
        let made = made.ok_or_else(|| self.fault(FaultKind::Type, at))?;
        self.charge(made.size() as u64, at)?;
        self.result(aml, t, k[k.len() - 1], made, at)
    }
}
