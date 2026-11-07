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

//! The cursor queue: one command in flight at most, each a single
//! descriptor the device reads, staged on the queue's own page. The device
//! answers nothing but the used entry. A command not used by CURSORQ_MS,
//! or used out of turn, takes the queue out of service for good: the chain
//! the device holds names the staging page, so nothing is staged there
//! again, and the control queue and its device are left as they are.

use core::cell::Cell;

use super::{avail, desc, layout::QueueLayout, used, wait};
use crate::clock::expired_after;
use crate::constants::{CURSORQ_INDEX, CURSORQ_MS, VQ_STAGING_OFFSET};
use crate::regs::Regs;

pub struct CursorQueue {
    pub layout: QueueLayout,
    /// The registers, with the cursor queue's doorbell.
    pub regs: Regs,
    next: Cell<u16>,
    out: Cell<bool>,
}

impl CursorQueue {
    pub fn new(layout: QueueLayout, regs: Regs) -> Self {
        Self { layout, regs, next: Cell::new(0), out: Cell::new(false) }
    }

    /// Whether a command was late or answered out of turn.
    pub fn is_out(&self) -> bool {
        self.out.get()
    }

    /// `request` given to the device and used by it.
    pub fn send(&self, request: &[u8]) -> Result<(), &'static str> {
        if self.is_out() {
            return Err("virtio-gpu: cursor queue out of service");
        }
        let layout = self.layout;
        if request.is_empty() || request.len() > layout.staging_len() {
            return Err("virtio-gpu: cursor command size");
        }
        for (at, b) in request.iter().enumerate() {
            layout.ring.w8(VQ_STAGING_OFFSET + at, *b);
        }
        let next = self.next.get();
        let head = next % layout.queue_size;
        let len = request.len() as u32;
        desc::write_desc(layout, head, layout.staging_device_addr(), len, 0, 0);
        avail::publish(layout, next, head);
        self.regs.notify(CURSORQ_INDEX);
        if let Err(e) = wait::used_moved(layout, next, expired_after(CURSORQ_MS)) {
            self.out.set(true);
            return Err(e);
        }
        let entry = used::read_entry(layout, next);
        let done = next.wrapping_add(1);
        self.next.set(done);
        if used::read_idx(layout) != done || entry.id != u32::from(head) {
            self.out.set(true);
            return Err("virtio-gpu: cursor used id mismatch");
        }
        Ok(())
    }
}
