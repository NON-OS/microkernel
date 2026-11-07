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
//! A scroll guest's two buffers, each in a pool of its own, under ids taken
//! one after another, as libwayland takes a client's new ids: the next one
//! not used, never a gap. Replaced whole as a client replaces its buffers
//! when its window changes size, the compositor holding one or not.

use super::conn::Conn;
use super::ids::SHM;
use super::msg::Msg;
use super::scroll_pixels::Buffer;

const BUFFER_DESTROY: u16 = 0;
const POOL_CREATE_BUFFER: u16 = 0;
const POOL_DESTROY: u16 = 1;
const SHM_CREATE_POOL: u16 = 0;
/// WL_SHM_FORMAT_XRGB8888.
const XRGB: u32 = 1;

pub struct Slot {
    pub buf: Buffer,
    pub pool: u32,
    pub id: u32,
    pub free: bool,
}

pub struct Chain {
    pub slots: [Option<Slot>; 2],
    next: u32,
    /// Buffers made since the start.
    pub made: u32,
}

impl Chain {
    /// No buffers yet; the first new id is `first`.
    pub fn new(first: u32) -> Chain {
        Chain { slots: [None, None], next: first, made: 0 }
    }

    /// The next new id.
    pub fn take(&mut self) -> u32 {
        let id = self.next;
        self.next = self.next.saturating_add(1);
        id
    }

    /// Two buffers `w` x `h` made and handed to the display; false when
    /// shared memory is refused.
    pub fn fill(&mut self, c: &Conn, (w, h): (u32, u32)) -> bool {
        for i in 0..2 {
            let Some(buf) = Buffer::new(w, h) else { return false };
            let (pool, id) = (self.take(), self.take());
            c.send_fd(&Msg::new(SHM, SHM_CREATE_POOL).u32(pool).u32(w * h * 4).bytes(), buf.fd);
            let row = w * 4;
            let at = Msg::new(pool, POOL_CREATE_BUFFER).u32(id).u32(0);
            c.send(&at.u32(w).u32(h).u32(row).u32(XRGB).bytes());
            self.slots[i] = Some(Slot { buf, pool, id, free: true });
            self.made = self.made.saturating_add(1);
        }
        true
    }

    /// Every buffer and pool destroyed and its memory unmapped and closed.
    pub fn drop_all(&mut self, c: &Conn) {
        for slot in self.slots.iter_mut() {
            if let Some(s) = slot.take() {
                c.send(&Msg::new(s.id, BUFFER_DESTROY).bytes());
                c.send(&Msg::new(s.pool, POOL_DESTROY).bytes());
            }
        }
    }

    /// The slot of a buffer the display gave back, if any.
    pub fn free_one(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.as_ref().is_some_and(|s| s.free))
    }

    /// `object` given back by the display, when it is one of these buffers.
    pub fn released(&mut self, object: u32) {
        let mine = self.slots.iter_mut().flatten().find(|s| s.id == object);
        if let Some(s) = mine {
            s.free = true;
        }
    }
}
