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

//! The integer operators with one operand: Not, FindSetLeftBit,
//! FindSetRightBit, ToBCD, FromBCD (sections 19.6.92, 19.6.49, 19.6.50,
//! 19.6.137, 19.6.51), and Increment and Decrement.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{from_bcd, to_bcd, Object};
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn unary(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let (a, target) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        let (x, w) = (self.int(aml, t, a)?, self.width(aml, t));
        let r = match op {
            Op::NOT => w.mask(!x),
            // One-based bit positions; zero when no bit is set.
            Op::FIND_SET_LEFT_BIT if x == 0 => 0,
            Op::FIND_SET_LEFT_BIT => 64 - x.leading_zeros() as u64,
            Op::FIND_SET_RIGHT_BIT if x == 0 => 0,
            Op::FIND_SET_RIGHT_BIT => x.trailing_zeros() as u64 + 1,
            Op::TO_BCD => to_bcd(x, w),
            _ => from_bcd(x, w).ok_or_else(|| self.fault(FaultKind::Type, at))?,
        };
        self.result(aml, t, target, Object::Integer(r), at)
    }

    /// Increment and Decrement (sections 19.6.63 and 19.6.30): read, step,
    /// store back, return the new value.
    pub(crate) fn step(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let target = aml.kid(t, node, 0);
        let x = self.int(aml, t, target)?;
        let w = self.width(aml, t);
        let r = match op {
            Op::INCREMENT => x.wrapping_add(1),
            _ => x.wrapping_sub(1),
        };
        self.result(aml, t, target, Object::Integer(w.mask(r)), at)
    }
}
