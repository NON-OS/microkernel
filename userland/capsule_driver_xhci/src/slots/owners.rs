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
//! Who configured each pipe of a slot, and who may use it. A device no
//! driver claimed an interface of is served to anyone; a shared one only to
//! claimants, each pipe only to the one that configured it, never
//! configured again by another.

use alloc::vec::Vec;

use super::asker::Asker;
use super::uses::{Pipe, Use};

/// Pipes recorded at once, over every slot.
const OWNERS: usize = 128;
pub const E_PERM: i32 = -1;
pub const E_BUSY: i32 = -16;

#[derive(Default)]
pub struct Owners {
    list: Vec<(u8, Pipe, u32)>,
}

impl Owners {
    /// Whether `who` may `u` on `slot`; a pipe it configures is its own.
    pub fn admit(&mut self, slot: u8, u: Use, who: Asker) -> Result<(), i32> {
        let Asker { pid, shared, holds } = who;
        let other = |o: Option<u32>| shared && o.is_some_and(|o| o != pid);
        match u {
            _ if shared && !holds => Err(E_PERM),
            Use::Control => Ok(()),
            Use::Transfer(p) if other(self.owner(slot, p)) => Err(E_PERM),
            Use::Transfer(_) => Ok(()),
            Use::Configure(p) if other(self.owner(slot, p)) => Err(E_BUSY),
            Use::Configure(p) if self.owner(slot, p).is_some() => Ok(()),
            Use::Configure(_) if self.list.len() >= OWNERS && shared => Err(E_BUSY),
            Use::Configure(_) if self.list.len() >= OWNERS => Ok(()),
            Use::Configure(p) => {
                self.list.push((slot, p, pid));
                Ok(())
            }
        }
    }

    /// A sharer left the slot: only pipes of drivers still holding a claim
    /// keep an owner, so a restarted driver can configure its pipes again.
    pub fn keep_holders(&mut self, slot: u8, holds: impl Fn(u32) -> bool) {
        self.list.retain(|o| o.0 != slot || holds(o.2));
    }

    /// The slot was disabled: its pipes go with it.
    pub fn forget(&mut self, slot: u8) {
        self.list.retain(|o| o.0 != slot);
    }

    fn owner(&self, slot: u8, p: Pipe) -> Option<u32> {
        self.list.iter().find(|o| o.0 == slot && o.1 == p).map(|o| o.2)
    }
}
