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

//! Disarming a device's runtime wake, newest first: the wake event off
//! when it was its last user, firmware told with `_DSW` or `_PSW`, the
//! power resources given back to the pool; for a device that has left,
//! the same without `_DSW`. And each Notify(device, 2) counted.

use super::pool::PowerPool;
use super::wake_method::wake_enable;
use super::wake_types::{Arm, GpeSwitch, RuntimeWake};
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;

impl RuntimeWake {
    /// False when the device was not armed.
    pub fn disarm(
        &mut self,
        (pool, aml, eval): (&mut PowerPool, &mut Aml, &mut Eval<'_>),
        gpes: &mut dyn GpeSwitch,
        dev: Id,
    ) -> bool {
        let Some(a) = self.take(gpes, dev) else { return false };
        let _ = wake_enable(aml, eval, dev, false, a.target);
        let _ = pool.release(aml, eval, &a.resources);
        true
    }

    /// The device has left: its event off with its last user and its
    /// resources given back, but nothing run on the device, which is gone.
    pub fn forget(
        &mut self,
        (pool, aml, eval): (&mut PowerPool, &mut Aml, &mut Eval<'_>),
        gpes: &mut dyn GpeSwitch,
        dev: Id,
    ) -> bool {
        let Some(a) = self.take(gpes, dev) else { return false };
        let _ = pool.release(aml, eval, &a.resources);
        true
    }

    fn take(&mut self, gpes: &mut dyn GpeSwitch, dev: Id) -> Option<Arm> {
        let a = self.armed.remove(self.armed.iter().position(|a| a.dev == dev)?);
        if !self.armed.iter().any(|o| o.gpe == a.gpe) {
            let _ = gpes.switch(a.gpe, false);
        }
        Some(a)
    }

    /// A Notify(device, 2): counted when the device is armed.
    pub fn woken(&mut self, dev: Id) -> bool {
        let a = self.armed.iter_mut().find(|a| a.dev == dev);
        a.map(|a| a.wakes = a.wakes.wrapping_add(1)).is_some()
    }

    /// The wakes counted since it was armed; None when it is not.
    pub fn wakes(&self, dev: Id) -> Option<u32> {
        self.armed.iter().find(|a| a.dev == dev).map(|a| a.wakes)
    }
}
