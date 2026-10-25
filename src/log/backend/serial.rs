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

//! The kernel's warnings on the serial line, for the attack suite's boot
//! only (nonos-capsule-attack): its report sets each refusal beside the
//! kernel's own line for it, [CAP-DENY] and the rest, which otherwise reach
//! only the screen and the RAM log. No other image echoes them.

use super::traits::LogBackend;
use crate::log::types::LogEntry;
use crate::sys::serial::Line;

pub struct SerialBackend;

impl LogBackend for SerialBackend {
    /// One line, handed to the port in one call: with SMP on, a line written
    /// in pieces is spliced with other CPUs' lines, and the checker reads
    /// lines whole (src/sys/serial/line.rs).
    fn write(&mut self, entry: &LogEntry) {
        Line::new()
            .str(b"[KLOG ")
            .str(entry.sev.as_str().as_bytes())
            .str(b"] ")
            .str(entry.msg.as_bytes())
            .end();
    }
}
