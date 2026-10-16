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

//! Runtime wake: a device its driver puts in a low-power state while the
//! system runs, armed to wake it. Arming takes `_PRW`'s power resources
//! through the shared pool, tells firmware with `_DSW` or `_PSW`, then
//! turns the wake event on when it is the event's first user; a target
//! deeper than `_S0W` is refused. Disarming undoes it newest first. Each
//! Notify(device, 2) while armed is counted for the driver to read.

use super::pool::PowerPool;
use super::state::DState;
use super::wake_method::{s0w, wake_enable};
use super::wake_types::{Arm, GpeSwitch, RuntimeWake, WakeError};
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;
use crate::suspend::read_prw;

impl RuntimeWake {
    pub fn arm(
        &mut self,
        (pool, aml, eval): (&mut PowerPool, &mut Aml, &mut Eval<'_>),
        gpes: &mut dyn GpeSwitch,
        dev: Id,
        target: DState,
    ) -> Result<(), WakeError> {
        self.disarm((pool, aml, eval), gpes, dev);
        let prw = read_prw(aml, eval, dev).ok_or(WakeError::NoEvent)?;
        let gpe = prw.gpe.ok_or(WakeError::NoEvent)?;
        let deepest = s0w(aml, eval, dev).ok_or(WakeError::BadS0w)?;
        if target > deepest {
            return Err(WakeError::TooDeep(deepest));
        }
        pool.acquire(aml, eval, &prw.resources).map_err(WakeError::Fault)?;
        let undo = |pool: &mut PowerPool, aml: &mut Aml, eval: &mut Eval<'_>| {
            let _ = wake_enable(aml, eval, dev, false, target);
            let _ = pool.release(aml, eval, &prw.resources);
        };
        if let Err(f) = wake_enable(aml, eval, dev, true, target) {
            undo(pool, aml, eval);
            return Err(WakeError::Fault(f));
        }
        if !self.armed.iter().any(|a| a.gpe == gpe) && !gpes.switch(gpe, true) {
            undo(pool, aml, eval);
            return Err(WakeError::Event);
        }
        self.armed.push(Arm { dev, gpe, resources: prw.resources, target, wakes: 0 });
        Ok(())
    }
}
