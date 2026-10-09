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

//! What a failed I/O command answers its client. A bare I/O error told an
//! installer on real hardware nothing; the controller's own status, or the
//! fact that the command never completed, tells which part of the path
//! refused it.

use core::ptr::read_volatile;

use super::queue::IoQueue;
use crate::admin::Completion;
use crate::error::NvmeError;
use crate::protocol::{device_status, E_IO, E_TIMEDOUT};

impl IoQueue {
    /// The reply status for `e`, the error a transfer, flush or settle just
    /// returned. A completion with an error status is the entry the wait just
    /// consumed, the slot behind the head, and it stays in the ring until the
    /// controller laps it, so its status is read back from there.
    pub fn failure_status(&self, e: NvmeError) -> i32 {
        match e {
            NvmeError::ControllerTimeout | NvmeError::ClockFailed => E_TIMEDOUT,
            NvmeError::AdminCommandFailed => {
                let entries = self.cursor.entries;
                let slot = (self.cursor.head + entries - 1) % entries;
                let size = core::mem::size_of::<Completion>() as u64;
                let at = self.cq.user_va() + slot as u64 * size;
                // SAFETY: `slot` is below the ring's entries, which the
                // completion queue's DMA region holds (nvm::wait).
                let c = unsafe { read_volatile(at as *const Completion) };
                device_status(c.status_code_type(), c.status_code())
            }
            _ => E_IO,
        }
    }
}
