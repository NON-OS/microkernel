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

//! Taking and giving back the Global Lock (section 5.2.10.1) for \_GL and
//! for fields whose LockRule is Lock. The capsule holds it at most once,
//! counted; the FACS word is changed only on the first take and last give.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn global_lock(&mut self, aml: &mut Aml, take: bool, at: u32) -> Result<(), Fault> {
        if take {
            if aml.rt.global_lock == 0 {
                self.lock_take(at)?;
            }
            aml.rt.global_lock += 1;
            return Ok(());
        }
        if aml.rt.global_lock == 0 {
            return Err(self.fault(FaultKind::Mutex, at));
        }
        aml.rt.global_lock -= 1;
        if aml.rt.global_lock == 0 {
            self.lock_give(at)?;
        }
        Ok(())
    }
}
