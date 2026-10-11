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

//! Release, Signal and Reset (sections 19.6.117, 19.6.126 and 19.6.119).

use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::namespace::Kind;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn sync_stmt(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<(), Fault> {
        let at = aml.node(t, node).at;
        let a = aml.kid(t, node, 0);
        if op == Op::RELEASE {
            let m = self.object_of(aml, t, a, Some(Kind::Mutex))?;
            return self.release_mutex(aml, m, at);
        }
        let e = self.object_of(aml, t, a, Some(Kind::Event))?;
        let n = aml.rt.events.entry(e).or_insert(0);
        *n = if op == Op::SIGNAL { n.saturating_add(1) } else { 0 };
        Ok(())
    }
}
