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

//! One queue pair as the fake controller serves it: the submissions it has
//! fetched up to, and the completion slot and phase it writes next.

use core::ptr::{read_volatile, write_volatile};

use crate::admin::{Completion, Submission};

#[derive(Clone, Copy, Default)]
pub(crate) struct Ring {
    pub sq: u64,
    pub cq: u64,
    pub entries: u16,
    head: u16,
    tail: u16,
    phase: bool,
    /// Each completion raises the interrupt (CDW11.IEN at creation).
    irq: bool,
}

impl Ring {
    pub fn new(sq: u64, cq: u64, entries: u16, irq: bool) -> Self {
        Self { sq, cq, entries, head: 0, tail: 0, phase: true, irq }
    }

    /// The next command the driver submitted, up to the tail it rang.
    pub fn fetch(&mut self, rung: u32) -> Option<Submission> {
        if self.sq == 0 || self.entries == 0 || u32::from(self.head) == rung {
            return None;
        }
        let at = self.sq + u64::from(self.head) * 64;
        self.head = (self.head + 1) % self.entries;
        // SAFETY: the driver's own submission ring, host memory from the
        // shim, and the slot is below its entries.
        Some(unsafe { read_volatile(at as *const Submission) })
    }

    /// Post the completion of `cid` on submission queue `sq_id`, with the
    /// status field `status` (its phase bit is the ring's) and `dw0`.
    pub fn complete(&mut self, sq_id: u16, cid: u16, status: u16, dw0: u32) {
        let status = (status & !1) | u16::from(self.phase);
        let c = Completion { dw0, dw1: 0, sq_head: self.head, sq_id, cid, status };
        let at = self.cq + u64::from(self.tail) * 16;
        // SAFETY: the driver's own completion ring, and the slot is below
        // its entries.
        unsafe { write_volatile(at as *mut Completion, c) };
        self.tail += 1;
        if self.tail == self.entries {
            self.tail = 0;
            self.phase = !self.phase;
        }
        if self.irq {
            nonos_libc::raise();
        }
    }
}
