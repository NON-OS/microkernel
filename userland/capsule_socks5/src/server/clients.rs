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

use alloc::vec::Vec;

use crate::conn::Conn;

use super::who::{pid_of, Who, STREAMS_PER_CALLER};

/// Concurrent SOCKS handshakes. One page load opens several at once for the
/// document, styles, scripts and images, so a single slot cannot serve even
/// one client.
pub const MAX_CLIENTS: usize = 32;

struct Slot {
    who: Who,
    conn: Conn,
}

/// Handshake state per caller, keyed on the pid the kernel attests at
/// delivery, so one capsule cannot drive another's handshake.
pub struct Clients {
    slots: Vec<Slot>,
}

impl Default for Clients {
    fn default() -> Self {
        Self::new()
    }
}

impl Clients {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// The handshake for `w`, started if this is its first message. `None`
    /// when the table is full, or its caller already holds as many streams
    /// as one caller may, which the caller answers by refusing the client
    /// rather than evicting somebody else's live session.
    pub fn get(&mut self, w: Who) -> Option<&mut Conn> {
        if let Some(i) = self.slots.iter().position(|s| s.who == w) {
            return Some(&mut self.slots[i].conn);
        }
        let held = self.slots.iter().filter(|s| pid_of(s.who) == pid_of(w)).count();
        if self.slots.len() == MAX_CLIENTS || held >= STREAMS_PER_CALLER {
            return None;
        }
        self.slots.push(Slot { who: w, conn: Conn::new() });
        self.slots.last_mut().map(|s| &mut s.conn)
    }

    /// Whether `w` holds a slot.
    pub fn holds(&self, w: Who) -> bool {
        self.slots.iter().any(|s| s.who == w)
    }

    /// Drop `w`'s handshake, freeing its slot for the next caller.
    pub fn drop_client(&mut self, w: Who) {
        self.slots.retain(|s| s.who != w);
    }

    /// The callers holding a slot whom `alive` says have ended. Only a caller
    /// resets its own conversation, so one that crashed kept its slot, its
    /// tunnel and the bytes held for it for good, and after MAX_CLIENTS of
    /// them the Nym route refused every program.
    pub fn ended(&self, alive: impl Fn(u32) -> bool) -> Vec<Who> {
        self.slots.iter().map(|s| s.who).filter(|&w| !alive(pid_of(w))).collect()
    }
}
