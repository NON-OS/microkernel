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

//! Mutexes (sections 19.6.2 and 19.6.117): owned by one evaluation, taken
//! again by it without waiting, and only in rising sync level order.

use crate::exec::{Fault, FaultKind, Holder, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;

/// Acquire's timeout that means "wait for ever".
pub const FOREVER: u64 = 0xFFFF;

impl<H: Host> Machine<'_, H> {
    /// Returns true if the wait timed out. The capsule runs one evaluation
    /// at a time, so a mutex held by another evaluation is one that
    /// evaluation never released: nothing can free it, so a timed wait
    /// sleeps out its timeout and a wait for ever is a fault.
    pub(crate) fn acquire_mutex(
        &mut self,
        aml: &mut Aml,
        m: Id,
        level: u8,
        timeout: u64,
        at: u32,
    ) -> Result<bool, Fault> {
        if self.held.last().is_some_and(|(_, l)| *l > level) {
            return Err(self.fault(FaultKind::Mutex, at));
        }
        match aml.rt.mutexes.get_mut(&m) {
            Some(h) if h.eval == self.eval => {
                h.depth += 1;
                Ok(false)
            }
            Some(_) if timeout >= FOREVER => Err(self.fault(FaultKind::Mutex, at)),
            Some(_) => {
                self.pause(timeout * 1000, at)?;
                Ok(true)
            }
            None => {
                aml.rt.mutexes.insert(m, Holder { eval: self.eval, depth: 1 });
                self.held.push((m, level));
                Ok(false)
            }
        }
    }

    pub(crate) fn release_mutex(&mut self, aml: &mut Aml, m: Id, at: u32) -> Result<(), Fault> {
        let mine = aml.rt.mutexes.get_mut(&m).filter(|h| h.eval == self.eval);
        let h = mine.ok_or_else(|| self.fault(FaultKind::Mutex, at))?;
        h.depth -= 1;
        if h.depth == 0 {
            aml.rt.mutexes.remove(&m);
            self.held.retain(|(id, _)| *id != m);
        }
        Ok(())
    }
}
