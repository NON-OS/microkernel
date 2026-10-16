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

//! Binding weighed against a device's standing, and the owner's answer to
//! a held device: it settles only the device on that slot holding that
//! ticket, so an answer meant for another device, or for one since
//! replaced on the slot, settles nothing.

use super::authorize::{Authority, Policy, Standing, E_ACCES};

impl Authority {
    /// Whether a bind on `slot` may go on; `None` when the device there is
    /// not yet judged and must be.
    pub fn admit(&self, slot: u8) -> Option<Result<(), i32>> {
        if self.policy == Policy::Allow {
            return Some(Ok(()));
        }
        match self.standing(slot)? {
            Standing::Authorized => Some(Ok(())),
            Standing::Held { .. } | Standing::Denied => Some(Err(E_ACCES)),
        }
    }

    /// True when the answer settled a held device.
    pub fn answer(&mut self, slot: u8, ticket: u32, allow: bool) -> bool {
        if self.standing(slot) != Some(Standing::Held { ticket }) {
            return false;
        }
        let settled = if allow { Standing::Authorized } else { Standing::Denied };
        self.set(slot, settled);
        true
    }
}
