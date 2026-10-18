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

//! The integer operators with two operands and a target (sections 19.6.3,
//! 19.6.5, 19.6.86 to 19.6.91, 19.6.123 to 19.6.126 and 19.6.155).

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    /// Results wrap at the width, as the hardware the language models does;
    /// a shift by the width or more leaves nothing.
    pub(crate) fn binary(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let (a, b, target) = (aml.kid(t, node, 0), aml.kid(t, node, 1), aml.kid(t, node, 2));
        let (x, y) = (self.int(aml, t, a)?, self.int(aml, t, b)?);
        let w = self.width(aml, t);
        let r = match op {
            Op::ADD => x.wrapping_add(y),
            Op::SUBTRACT => x.wrapping_sub(y),
            Op::MULTIPLY => x.wrapping_mul(y),
            Op::AND => x & y,
            Op::NAND => !(x & y),
            Op::OR => x | y,
            Op::NOR => !(x | y),
            Op::XOR => x ^ y,
            Op::SHIFT_LEFT => x.checked_shl(y.min(64) as u32).unwrap_or(0),
            Op::SHIFT_RIGHT => x.checked_shr(y.min(64) as u32).unwrap_or(0),
            _ => x.checked_rem(y).ok_or_else(|| self.fault(FaultKind::DivideByZero, at))?,
        };
        self.result(aml, t, target, Object::Integer(w.mask(r)), at)
    }

    /// Divide: the remainder to the first target, the quotient to the second
    /// and as the result (section 19.6.32).
    pub(crate) fn divide(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let k = aml.kids(t, node);
        let (x, y) = (self.int(aml, t, k[0])?, self.int(aml, t, k[1])?);
        let q = x.checked_div(y).ok_or_else(|| self.fault(FaultKind::DivideByZero, at))?;
        self.result(aml, t, k[2], Object::Integer(x % y), at)?;
        self.result(aml, t, k[3], Object::Integer(q), at)
    }
}
