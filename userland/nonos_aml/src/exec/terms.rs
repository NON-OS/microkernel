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

//! Running a TermList, and the control statements that steer one.

use super::{Fault, Flow, Host, Machine};
use crate::load::Aml;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn terms(&mut self, aml: &mut Aml, t: u16, terms: u32) -> Result<Flow, Fault> {
        for i in 0..aml.kid_count(t, terms) {
            let k = aml.kid(t, terms, i);
            match self.stmt(aml, t, k)? {
                Flow::Normal => {}
                other => return Ok(other),
            }
        }
        Ok(Flow::Normal)
    }

    /// If, then Else when the predicate is zero (section 19.6.60).
    pub(crate) fn if_(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Flow, Fault> {
        let pred = aml.kid(t, node, 0);
        if self.int(aml, t, pred)? != 0 {
            let then = aml.kid(t, node, 1);
            return self.terms(aml, t, then);
        }
        if aml.kid_count(t, node) < 3 {
            return Ok(Flow::Normal);
        }
        let els = aml.kid(t, node, 2);
        let body = aml.kid(t, els, 0);
        self.terms(aml, t, body)
    }

    /// While (section 19.6.153). Each pass evaluates the predicate, and every
    /// evaluation is a step, so a loop that never ends runs out of budget.
    pub(crate) fn while_(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Flow, Fault> {
        let (pred, body) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        while self.int(aml, t, pred)? != 0 {
            match self.terms(aml, t, body)? {
                Flow::Break => break,
                Flow::Normal | Flow::Continue => {}
                ret => return Ok(ret),
            }
        }
        Ok(Flow::Normal)
    }
}
