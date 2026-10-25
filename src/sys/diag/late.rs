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


//! The late call: the caller's own deadline ran out before the kernel's
//! default (ipc/call/verdict.rs). Its own site, so late lines never spend
//! the starvation counter. Endpoint name and timing only, no pids.

use super::{serial, trail, Site};

impl Site {
    /// `what` gave no answer within the caller's `elapsed_ms`; `dropped`
    /// late replies have been discarded by the correlation check so far.
    pub fn late(&self, what: &str, elapsed_ms: u64, dropped: u64) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" late ");
        serial::print_str(what);
        serial::print(b" after ");
        serial::print_dec(elapsed_ms);
        serial::print(b" ms dropped=");
        serial::print_dec(dropped);
        trail(n);
    }
}
