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

//! A caller's short calls that a server let pass for the kernel's default
//! wait (ipc/call/silence.rs), said as unanswered. Names and counts only,
//! no pids.

use super::{serial, trail, Site};

impl Site {
    /// `what` answered none of a caller's calls in time for the kernel's
    /// default wait; `dropped` late replies discarded since boot.
    pub fn silent_for(&self, what: &str, dropped: u64) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" unanswered ");
        serial::print_str(what);
        serial::print(b" silent 5 s, late replies dropped=");
        serial::print_dec(dropped);
        trail(n);
    }

    /// A pair evicted from the full silence table, reported at once, by port.
    pub fn silent(&self, port: u32) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" unanswered port=");
        serial::print_dec(u64::from(port));
        serial::print(b" (silent, table full)");
        trail(n);
    }
}
