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

//! Acquire, Release, Wait, Signal and Reset (sections 19.6.2, 19.6.117,
//! 19.6.152, 19.6.126 and 19.6.119).

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::Kind;
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    /// Acquire and Wait return true (Ones) when they timed out.
    pub(crate) fn sync_expr(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let (a, b) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        let w = self.width(aml, t);
        let timed_out = if op == Op::ACQUIRE {
            let m = self.object_of(aml, t, a, Some(Kind::Mutex))?;
            let level = self.sync_level(aml, m, at)?;
            let timed_out = self.acquire_mutex(aml, m, level, aml.imm(t, b), at)?;
            if !timed_out && self.is_global_lock(aml, m) {
                self.global_lock(aml, true, at)?;
            }
            timed_out
        } else {
            let e = self.object_of(aml, t, a, Some(Kind::Event))?;
            let timeout = self.int(aml, t, b)?;
            match aml.rt.events.get_mut(&e) {
                Some(n) if *n > 0 => {
                    *n -= 1;
                    false
                }
                // Nothing in this evaluation can signal while it waits.
                _ if timeout >= super::mutex::FOREVER => {
                    return Err(self.fault(FaultKind::Time, at))
                }
                _ => self.pause(timeout * 1000, at).map(|_| true)?,
            }
        };
        Ok(Object::Integer(if timed_out { w.ones() } else { 0 }))
    }
}
