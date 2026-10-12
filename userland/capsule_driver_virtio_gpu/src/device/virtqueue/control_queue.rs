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

//! The control queue: one command in flight at most, counted by the driver,
//! and a named state for a device that stopped answering (plan/50 C1).

use core::cell::Cell;

use nonos_virtio::common::reset_until;

use super::{layout::QueueLayout, submit};
use crate::clock::expired_after;
use crate::constants::{RESET_MS, VQ_STAGING_OFFSET};
use crate::regs::Regs;

pub struct ControlQueue {
    pub layout: QueueLayout,
    pub regs: Regs,
    /// Commands published: the driver's own count.
    pub(super) next: Cell<u16>,
    /// Set once a command timed out or was answered out of turn and the
    /// device was reset. No command goes out after that: the chain the
    /// device was given named the staging page the next one would reuse.
    reset: Cell<bool>,
}

impl ControlQueue {
    pub fn new(layout: QueueLayout, regs: Regs) -> Self {
        Self { layout, regs, next: Cell::new(0), reset: Cell::new(false) }
    }

    pub fn submit(
        &self,
        request: &[u8],
        resp_len: u32,
    ) -> Result<submit::SubmitOutput, &'static str> {
        submit::submit_sync(self, request, resp_len)
    }

    /// The answer, from the staging page after the `request_len` bytes of
    /// the request it answers.
    pub fn read_response(&self, request_len: usize, dst: &mut [u8]) {
        let from = VQ_STAGING_OFFSET.saturating_add(request_len);
        for (i, b) in dst.iter_mut().enumerate() {
            *b = self.layout.ring.r8(from.saturating_add(i));
        }
    }

    /// Whether the device was reset and the queue is out of service.
    pub fn is_reset(&self) -> bool {
        self.reset.get()
    }

    /// Stop the device (status 0, read back by the clock), so the chain in
    /// flight is dropped rather than run with the next command's bytes, and
    /// take the queue out of service. Also how a failed recovery try ends.
    pub fn reset_device(&self) {
        let _ = reset_until(&self.regs.common, expired_after(RESET_MS));
        self.reset.set(true);
    }
}
