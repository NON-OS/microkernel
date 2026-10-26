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

//! The kernel's warnings on the serial line, [CAP-DENY], [IPC-DENY] and the
//! rest, which otherwise reach only the screen and the RAM log: on the
//! attack suite's boot, whose report sets each refusal beside the kernel's
//! own line for it, and on the images that show capsule output on serial
//! (`capsule-serial-debug`, which no release loader carries: config.nix
//! and checks.nix `profile-serial`). Each tag's echo is budgeted
//! (serial_budget.rs); the RAM log keeps every line. Two lines are also
//! written to serial by their sites, block_device/kept.rs and data/errno.rs:
//! on a dev image they show twice, and on a release image that println is
//! their only serial line, so neither copy is to be removed.

#[path = "serial_budget.rs"]
mod serial_budget;

use super::traits::LogBackend;
use crate::log::types::LogEntry;
use crate::sys::serial::Line;
use serial_budget::{tag, Budget, Echo};

pub struct SerialBackend {
    budget: Budget,
}

impl SerialBackend {
    pub const fn new() -> Self {
        SerialBackend { budget: Budget::new() }
    }
}

impl LogBackend for SerialBackend {
    /// One line, handed to the port in one call: with SMP on, a line written
    /// in pieces is spliced with other CPUs' lines, and the checker reads
    /// lines whole (src/sys/serial/line.rs).
    fn write(&mut self, entry: &LogEntry) {
        let msg = entry.msg.as_bytes();
        match self.budget.admit(msg, crate::time::timestamp_millis()) {
            Echo::Not => return,
            Echo::After(n) => {
                let named = if tag(msg).is_empty() { &b"untagged"[..] } else { tag(msg) };
                Line::new()
                    .str(b"[LOG] ")
                    .dec(u64::from(n))
                    .str(b" lines of ")
                    .str(named)
                    .str(b" not echoed")
                    .end();
            }
            Echo::Line => {}
        }
        Line::new().str(b"[KLOG ").str(entry.sev.as_str().as_bytes()).str(b"] ").str(msg).end();
    }
}
