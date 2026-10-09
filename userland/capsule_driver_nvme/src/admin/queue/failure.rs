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

use core::ptr::read_volatile;

use super::types::AdminQueue;
use crate::admin::Completion;
use crate::error::{reason, NvmeError};
use crate::log::{emit, Line};

impl AdminQueue {
    /// Say on the console which admin command failed and how. A failed
    /// command's completion is the entry the wait just consumed, the slot
    /// behind the head, and it stays in the ring until the controller laps
    /// it, so its status is read back from there.
    pub(super) fn say_failed(&self, what: &str, e: NvmeError) {
        let mut line = Line::new();
        line.text(b"admin ").text(what.as_bytes());
        match e {
            NvmeError::AdminCommandFailed => {
                let entries = self.cursor.entries;
                let slot = (self.cursor.head + entries - 1) % entries;
                let size = core::mem::size_of::<Completion>() as u64;
                let at = self.cq.user_va() + slot as u64 * size;
                let c = unsafe { read_volatile(at as *const Completion) };
                line.text(b" failed: sct ")
                    .hex(c.status_code_type() as u64, 1)
                    .text(b" sc ")
                    .hex(c.status_code() as u64, 2)
                    .text(if c.do_not_retry() { &b" dnr"[..] } else { &b""[..] })
                    .text(b" (status ")
                    .hex(c.status as u64, 4)
                    .text(b")");
            }
            NvmeError::ControllerTimeout => {
                line.text(b": no completion within the admin timeout");
            }
            other => {
                line.text(b": ").text(reason(other).as_bytes());
            }
        }
        emit(&mut line);
    }
}
