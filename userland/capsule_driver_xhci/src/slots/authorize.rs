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

//! Whether a device may be bound, decided once per device before any of
//! its interfaces is: allowed as before, held for the owner's answer, or
//! refused. Its standing goes with the slot, and a new device on the slot
//! is judged afresh (server/mux_enable.rs forgets it).

use alloc::vec::Vec;

use super::asks::asks;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Policy {
    /// Every device bound, as before this gate.
    Allow,
    /// A keyboard or a device of several classes held until answered.
    Ask,
    /// No device bound.
    Deny,
}

/// The owner's to choose (an owner row). Allow, so nothing changes on any
/// machine until the owner says otherwise.
pub const DEFAULT_POLICY: Policy = Policy::Allow;
pub const E_ACCES: i32 = -13;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Standing {
    Authorized,
    Held { ticket: u32 },
    Denied,
}

pub struct Authority {
    pub policy: Policy,
    pub(super) list: Vec<(u8, Standing)>,
    tickets: u32,
}

impl Authority {
    pub fn new(policy: Policy) -> Self {
        Self { policy, list: Vec::new(), tickets: 0 }
    }

    /// Judge the device on `slot` by its first configuration, `None` when
    /// it could not be read (then it is held or refused, never allowed).
    pub fn judge(&mut self, slot: u8, config: Option<&[u8]>) -> Standing {
        let standing = match self.policy {
            Policy::Allow => Standing::Authorized,
            Policy::Deny => Standing::Denied,
            Policy::Ask if config.is_some_and(|c| !asks(c)) => Standing::Authorized,
            Policy::Ask => {
                self.tickets = self.tickets.wrapping_add(1).max(1);
                Standing::Held { ticket: self.tickets }
            }
        };
        self.set(slot, standing);
        standing
    }
}
