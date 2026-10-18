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

//! Binding weighed against a configuration's standing, and the owner's
//! answer to a held one: it settles only the configuration on that slot
//! holding that ticket, so an answer meant for another device, or for one
//! since replaced on the slot, settles nothing.

use super::authorize::{Authority, Policy, Standing, E_ACCES};

fn verdict(s: Standing) -> Result<(), i32> {
    match s {
        Standing::Authorized => Ok(()),
        Standing::Held { .. } | Standing::Denied => Err(E_ACCES),
    }
}

impl Authority {
    /// Whether a pipe or hub bind on `slot` may go on, by its current
    /// configuration; `None` when none is judged yet.
    pub fn admit(&self, slot: u8) -> Option<Result<(), i32>> {
        if self.policy == Policy::Allow {
            return Some(Ok(()));
        }
        self.standing(slot).map(verdict)
    }

    /// Whether SET_CONFIGURATION to `value` may go on; `None` when that
    /// configuration is not judged yet and must be. Value 0 unconfigures
    /// the device and binds nothing.
    pub fn admit_set(&mut self, slot: u8, value: u8) -> Option<Result<(), i32>> {
        if self.policy == Policy::Allow || value == 0 {
            return Some(Ok(()));
        }
        self.current(slot, value).map(verdict)
    }

    /// True when the answer settled a held configuration.
    pub fn answer(&mut self, slot: u8, ticket: u32, allow: bool) -> bool {
        let held = Standing::Held { ticket };
        let Some(e) = self.list.iter_mut().find(|e| e.0 == slot && e.2 == held) else {
            return false;
        };
        e.2 = if allow { Standing::Authorized } else { Standing::Denied };
        true
    }
}
