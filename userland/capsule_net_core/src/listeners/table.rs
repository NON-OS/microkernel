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

//! Passive opens, pure: which client listens on which port, the socket
//! waiting for the next SYN there, and the connections it has taken that no
//! ACCEPT has yet handed out. Listener handles sit above every connection
//! handle, so CLOSE tells the two apart by the number alone.

use alloc::vec::Vec;

/// Listener handles are `LISTENER_BASE + slot + 1`; connection handles are
/// small and 1-based, so the two ranges never meet.
pub const LISTENER_BASE: u32 = 0x4000_0000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    PortInUse,
    Full,
}

#[derive(Clone, Copy)]
pub struct Listener<H, const B: usize> {
    pub owner: u32,
    pub port: u16,
    pub socket: H,
    queue: [Option<H>; B],
}

impl<H: Copy + PartialEq, const B: usize> Listener<H, B> {
    /// Queue a connection the listening socket took. `false` when the
    /// backlog is full; the caller leaves the connection where it is.
    pub fn push(&mut self, h: H) -> bool {
        match self.queue.iter_mut().find(|s| s.is_none()) {
            Some(slot) => {
                *slot = Some(h);
                true
            }
            None => false,
        }
    }

    pub fn has_room(&self) -> bool {
        self.queue.iter().any(|s| s.is_none())
    }

    /// The oldest queued connection `ready` says is open, taken out. Older
    /// ones still in their handshake keep their place.
    pub fn take(&mut self, ready: impl Fn(H) -> bool) -> Option<H> {
        let at = self.queue.iter().position(|s| s.is_some_and(&ready))?;
        let h = self.queue[at].take();
        self.queue[at..].rotate_left(1);
        h
    }

    /// Whether an ACCEPT now would hand out a connection.
    pub fn any_ready(&self, ready: impl Fn(H) -> bool) -> bool {
        self.queue.iter().flatten().any(|h| ready(*h))
    }

    /// Put back at the front a connection `take` gave out that could not be
    /// handed on. `false` if the backlog filled in between.
    pub fn restore(&mut self, h: H) -> bool {
        if !self.has_room() {
            return false;
        }
        self.queue.rotate_right(1);
        self.queue[0] = Some(h);
        true
    }

    /// Take out every queued connection `keep` refuses.
    pub fn drop_queued(&mut self, keep: impl Fn(H) -> bool) -> Vec<H> {
        let mut gone = Vec::new();
        let mut kept = [None; B];
        let mut n = 0;
        for h in self.queue.iter().flatten() {
            if keep(*h) {
                kept[n] = Some(*h);
                n += 1;
            } else {
                gone.push(*h);
            }
        }
        self.queue = kept;
        gone
    }

    /// The listening socket and every queued connection.
    pub fn all_sockets(&self) -> Vec<H> {
        let mut all = Vec::with_capacity(B + 1);
        all.push(self.socket);
        all.extend(self.queue.iter().flatten());
        all
    }
}

pub struct Listeners<H, const N: usize, const B: usize> {
    slots: [Option<Listener<H, B>>; N],
}

impl<H: Copy + PartialEq, const N: usize, const B: usize> Listeners<H, N, B> {
    pub const fn new() -> Self {
        Self { slots: [None; N] }
    }

    fn slot_of(handle: u32) -> Option<usize> {
        let at = handle.checked_sub(LISTENER_BASE)?.checked_sub(1)? as usize;
        (at < N).then_some(at)
    }

    pub fn is_listener(handle: u32) -> bool {
        Self::slot_of(handle).is_some()
    }

    /// One listener on a port, whoever holds it.
    pub fn open(&mut self, owner: u32, port: u16, socket: H) -> Result<u32, Refusal> {
        if self.slots.iter().flatten().any(|l| l.port == port) {
            return Err(Refusal::PortInUse);
        }
        let at = self.slots.iter().position(|s| s.is_none()).ok_or(Refusal::Full)?;
        self.slots[at] = Some(Listener { owner, port, socket, queue: [None; B] });
        Ok(LISTENER_BASE + at as u32 + 1)
    }

    pub fn get_mut(&mut self, handle: u32, owner: u32) -> Option<&mut Listener<H, B>> {
        self.slots.get_mut(Self::slot_of(handle)?)?.as_mut().filter(|l| l.owner == owner)
    }

    /// Close a listener its owner holds: its socket and its queue, to let go.
    pub fn close(&mut self, handle: u32, owner: u32) -> Option<Vec<H>> {
        let slot = self.slots.get_mut(Self::slot_of(handle)?)?;
        if !slot.is_some_and(|l| l.owner == owner) {
            return None;
        }
        slot.take().map(|l| l.all_sockets())
    }

    pub fn for_each_mut(&mut self, mut f: impl FnMut(&mut Listener<H, B>)) {
        for l in self.slots.iter_mut().flatten() {
            f(l);
        }
    }

    pub fn any(&self) -> bool {
        self.slots.iter().any(|s| s.is_some())
    }

    /// Take out every listener whose owner `alive` says has ended.
    pub fn take_ended(&mut self, alive: impl Fn(u32) -> bool) -> Vec<H> {
        let mut gone = Vec::new();
        for slot in self.slots.iter_mut() {
            if slot.is_some_and(|l| !alive(l.owner)) {
                gone.extend(slot.take().map(|l| l.all_sockets()).unwrap_or_default());
            }
        }
        gone
    }

    /// Take out every listener, for a route that is no longer Direct.
    pub fn take_all(&mut self) -> Vec<H> {
        self.take_ended(|_| false)
    }

    /// Forget every listener without touching the stack, as for a rebuilt one.
    pub fn forget_all(&mut self) {
        self.slots = [None; N];
    }
}
