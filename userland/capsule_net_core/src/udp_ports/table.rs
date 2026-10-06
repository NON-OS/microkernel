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

//! The UDP ports net.core holds bound for its clients, each with the client
//! that bound it. Generic over the socket handle, so host proofs run the same
//! table with a handle of their own.

use alloc::vec::Vec;

#[derive(Clone, Copy)]
struct Entry<H> {
    owner_pid: u32,
    local_port: u16,
    handle: H,
}

/// `N` bound ports, each found by its binder's pid and the port.
pub struct Ports<H, const N: usize> {
    slots: [Option<Entry<H>>; N],
}

impl<H: Copy, const N: usize> Ports<H, N> {
    /// The most ports one client holds bound, so no one client can take
    /// every port from the rest.
    pub const PER_OWNER: usize = N / 2;

    pub const fn new() -> Self {
        Self { slots: [None; N] }
    }

    pub fn insert(&mut self, owner_pid: u32, local_port: u16, handle: H) -> bool {
        if self.held_by(owner_pid) >= Self::PER_OWNER {
            return false;
        }
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(Entry { owner_pid, local_port, handle });
                return true;
            }
        }
        false
    }

    pub fn get(&self, owner_pid: u32, local_port: u16) -> Option<H> {
        self.slots.iter().find_map(|s| {
            s.as_ref().and_then(|e| {
                if e.owner_pid == owner_pid && e.local_port == local_port {
                    Some(e.handle)
                } else {
                    None
                }
            })
        })
    }

    pub fn remove(&mut self, owner_pid: u32, local_port: u16) {
        for slot in self.slots.iter_mut() {
            let matches = slot
                .as_ref()
                .map_or(false, |e| e.owner_pid == owner_pid && e.local_port == local_port);
            if matches {
                *slot = None;
                return;
            }
        }
    }

    /// How many ports `owner_pid` holds bound.
    pub fn held_by(&self, owner_pid: u32) -> usize {
        self.slots.iter().flatten().filter(|e| e.owner_pid == owner_pid).count()
    }

    /// Take out every port whose client `alive` says has ended, for the
    /// caller to unbind as that client's own unbind would have. Only the
    /// client can unbind its port, so one that ended held it for good.
    pub fn take_ended(&mut self, alive: impl Fn(u32) -> bool) -> Vec<H> {
        let mut gone = Vec::new();
        for slot in self.slots.iter_mut() {
            if let Some(entry) = *slot {
                if !alive(entry.owner_pid) {
                    gone.push(entry.handle);
                    *slot = None;
                }
            }
        }
        gone
    }

    /// Forget every port without touching the stack. For a stack that was
    /// rebuilt: each handle here names a socket in the old socket set.
    pub fn forget_all(&mut self) {
        self.slots = [None; N];
    }
}
