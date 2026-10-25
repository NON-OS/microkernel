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

//! An answer the kernel would not hand to the call it was for
//! (ipc/reply_dest.rs). The service's name and the reason only, no pids.

use super::{serial, trail, Site};

impl Site {
    /// `what` answered a call, and its answer was refused for `why`.
    pub fn reply_refused(&self, what: &str, why: &'static str) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" reply refused ");
        serial::print_str(what);
        serial::print(b": ");
        serial::print_str(why);
        trail(n);
    }
}
