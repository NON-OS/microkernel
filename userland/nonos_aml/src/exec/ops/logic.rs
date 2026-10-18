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

//! The logical operators (sections 19.6.69 to 19.6.75). True is Ones at the
//! width and false is Zero; both operands are evaluated, left to right.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{compare, Object};
use crate::op::Op;
use core::cmp::Ordering;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn logic(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let w = self.width(aml, t);
        let truth = |b: bool| Object::Integer(if b { w.ones() } else { 0 });
        let a = aml.kid(t, node, 0);
        if op == Op::LNOT {
            return Ok(truth(self.int(aml, t, a)? == 0));
        }
        let b = aml.kid(t, node, 1);
        if op == Op::LAND || op == Op::LOR {
            let (x, y) = (self.int(aml, t, a)? != 0, self.int(aml, t, b)? != 0);
            return Ok(truth(if op == Op::LAND { x && y } else { x || y }));
        }
        let (x, y) = (self.value(aml, t, a)?, self.value(aml, t, b)?);
        let ord = compare(&x, &y, w).ok_or_else(|| self.fault(FaultKind::Type, at))?;
        Ok(truth(match op {
            Op::LEQUAL => ord == Ordering::Equal,
            Op::LGREATER => ord == Ordering::Greater,
            _ => ord == Ordering::Less,
        }))
    }
}
