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

//! Who enabled each slot. On a slot no interface of is claimed, that driver
//! is the only one served (gate.rs). The kernel (pid 0) is not held to it,
//! and a holder the kernel says is gone is replaced by the next driver that
//! asks, so a restarted driver can give back what its last life left.

use alloc::vec::Vec;

#[derive(Default)]
pub struct Holders {
    list: Vec<(u8, u32)>,
}

impl Holders {
    pub fn set(&mut self, slot: u8, pid: u32) {
        self.forget(slot);
        self.list.push((slot, pid));
    }

    pub fn forget(&mut self, slot: u8) {
        self.list.retain(|h| h.0 != slot);
    }

    pub fn holder(&self, slot: u8) -> Option<u32> {
        self.list.iter().find(|h| h.0 == slot).map(|h| h.1)
    }

    /// Whether `pid` may act on `slot`. A slot with no holder is not yet
    /// enabled, and the request is refused further on as invalid.
    pub fn admits(&mut self, slot: u8, pid: u32, alive: &impl Fn(u32) -> bool) -> bool {
        match self.holder(slot) {
            _ if pid == 0 => true,
            None => true,
            Some(h) if h == pid => true,
            Some(h) if !alive(h) => {
                self.set(slot, pid);
                true
            }
            Some(_) => false,
        }
    }
}
