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

//! Sleep and Stall (sections 19.6.128 and 19.6.129), and the waits Acquire
//! and Wait time out with. Every microsecond is counted before it is spent.

use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    /// Gives up the processor for `us` microseconds, rounded up to whole
    /// milliseconds.
    pub(crate) fn pause(&mut self, us: u64, at: u32) -> Result<(), Fault> {
        self.spend(us, at)?;
        self.host.sleep(us.div_ceil(1000));
        Ok(())
    }

    /// Sleep's operand is milliseconds; Stall's is microseconds, which the
    /// specification asks firmware to keep short. A longer stall is honoured,
    /// not shortened, because firmware times hardware with it; the budget is
    /// what bounds it.
    pub(crate) fn wait_stmt(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        op: Op,
    ) -> Result<(), Fault> {
        let at = aml.node(t, node).at;
        let a = aml.kid(t, node, 0);
        let v = self.int(aml, t, a)?;
        if op == Op::SLEEP {
            return self.pause(v.saturating_mul(1000), at);
        }
        self.spend(v, at)?;
        self.host.stall(v);
        Ok(())
    }
}
