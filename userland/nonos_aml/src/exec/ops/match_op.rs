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

//! Match (section 19.6.80): the first package element, from a start index,
//! that meets both comparisons.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::{compare, Object, Width};
use core::cmp::Ordering;

/// MatchOpcode: MTR, MEQ, MLE, MLT, MGE, MGT, comparing the element on the
/// left to the operand on the right.
fn meets(op: u64, elem: &Object, operand: &Object, w: Width) -> Option<bool> {
    if op == 0 {
        return Some(true);
    }
    let ord = compare(elem, operand, w)?;
    Some(match op {
        1 => ord == Ordering::Equal,
        2 => ord != Ordering::Greater,
        3 => ord == Ordering::Less,
        4 => ord != Ordering::Less,
        5 => ord == Ordering::Greater,
        _ => return None,
    })
}

impl<H: Host> Machine<'_, H> {
    /// Ones if no element matches. An element that is not an integer, string
    /// or buffer cannot be compared and is passed over; an unknown
    /// MatchOpcode is a fault.
    pub(crate) fn match_op(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let k = aml.kids(t, node);
        let Object::Package(p) = self.value(aml, t, k[0])? else {
            return Err(self.fault(FaultKind::Type, at));
        };
        let (op1, op2) = (aml.imm(t, k[1]), aml.imm(t, k[3]));
        if op1 > 5 || op2 > 5 {
            return Err(self.fault(FaultKind::Type, at));
        }
        let (v1, v2) = (self.value(aml, t, k[2])?, self.value(aml, t, k[4])?);
        let start = self.int(aml, t, k[5])?;
        let w = self.width(aml, t);
        let elems = p.borrow().clone();
        for (i, e) in elems.iter().enumerate().skip(usize::try_from(start).unwrap_or(usize::MAX)) {
            self.tick(at)?;
            let e = self.deref(aml, e.clone(), at)?;
            if meets(op1, &e, &v1, w) == Some(true) && meets(op2, &e, &v2, w) == Some(true) {
                return Ok(Object::Integer(i as u64));
            }
        }
        Ok(Object::Integer(w.ones()))
    }
}
