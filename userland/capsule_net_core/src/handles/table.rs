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

//! The TCP connections net.core holds for its clients, each with the client
//! that opened it. Generic over the socket handle, so host proofs run the
//! same table with a handle of their own.

use alloc::vec::Vec;

/// `N` connections, each the opener's pid and the stack's socket handle.
pub struct Handles<H, const N: usize> {
    slots: [Option<(u32, H)>; N],
}

impl<H: Copy, const N: usize> Handles<H, N> {
    /// The most connections one client holds, so no one client can take
    /// every connection from the rest.
    pub const PER_OWNER: usize = N / 2;

    pub const fn new() -> Self {
        Self { slots: [None; N] }
    }

    // Handles are 1-based: 0 is reserved as the null handle so a caller can use zero
    // as a sentinel (net.sockets stores a transport handle and treats 0 as "not
    // connected"). The external handle is `slot index + 1`; get/free undo the offset.
    pub fn alloc(&mut self, owner_pid: u32, handle: H) -> Option<u32> {
        if self.held_by(owner_pid) >= Self::PER_OWNER {
            return None;
        }
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some((owner_pid, handle));
                return Some(i as u32 + 1);
            }
        }
        None
    }

    pub fn get(&self, index: u32, sender_pid: u32) -> Option<H> {
        if index == 0 {
            return None;
        }
        self.slots
            .get((index - 1) as usize)
            .and_then(|s| s.and_then(|(owner, h)| if owner == sender_pid { Some(h) } else { None }))
    }

    pub fn free(&mut self, index: u32, sender_pid: u32) {
        if index == 0 {
            return;
        }
        if let Some(slot) = self.slots.get_mut((index - 1) as usize) {
            if matches!(slot, Some((owner, _)) if *owner == sender_pid) {
                *slot = None;
            }
        }
    }

    /// How many connections `owner_pid` holds.
    pub fn held_by(&self, owner_pid: u32) -> usize {
        self.slots.iter().flatten().filter(|(owner, _)| *owner == owner_pid).count()
    }

    /// Take out every connection whose client `alive` says has ended, for
    /// the caller to let go of as that client's close would have. Only the
    /// client can close its connection, so one that ended without closing
    /// held its place, and its peer's connection, for good.
    pub fn take_ended(&mut self, alive: impl Fn(u32) -> bool) -> Vec<H> {
        let mut gone = Vec::new();
        for slot in self.slots.iter_mut() {
            if let Some((owner, handle)) = *slot {
                if !alive(owner) {
                    gone.push(handle);
                    *slot = None;
                }
            }
        }
        gone
    }

    /// Forget every connection without touching the stack. For a stack that
    /// was rebuilt: each handle here names a socket in the old socket set.
    pub fn forget_all(&mut self) {
        self.slots = [None; N];
    }
}
