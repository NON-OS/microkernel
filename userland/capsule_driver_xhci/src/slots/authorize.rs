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

//! Whether a device may be bound, decided for each configuration it is set
//! to: allowed as before, held for the owner's answer, or refused. One set
//! to another configuration after a harmless one is judged again for it,
//! and a new device on the slot afresh (server/mux_enable.rs).

use alloc::vec::Vec;

use super::asks::asks;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Policy {
    /// Every device bound, as before; Ask holds a HID or several-class
    /// configuration until answered; Deny binds nothing.
    Allow,
    Ask,
    Deny,
}

/// The owner's to choose (owner Decision 12). Allow until the owner says
/// otherwise, so nothing changes on any machine.
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
    /// Slot, bConfigurationValue, standing; a slot's last is its current.
    pub(super) list: Vec<(u8, u8, Standing)>,
    tickets: u32,
}

impl Authority {
    pub fn new(policy: Policy) -> Self {
        Self { policy, list: Vec::new(), tickets: 0 }
    }

    /// Judge configuration `value` on `slot` by its bytes, `None` when
    /// unread (then never allowed); it becomes the slot's current one.
    pub fn judge(&mut self, slot: u8, value: u8, config: Option<&[u8]>) -> Standing {
        let standing = match self.policy {
            Policy::Allow => Standing::Authorized,
            Policy::Deny => Standing::Denied,
            Policy::Ask if config.is_some_and(|c| !asks(c)) => Standing::Authorized,
            Policy::Ask => {
                self.tickets = self.tickets.wrapping_add(1).max(1);
                Standing::Held { ticket: self.tickets }
            }
        };
        self.set(slot, value, standing);
        standing
    }
}
