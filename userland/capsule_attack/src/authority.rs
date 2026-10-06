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

//! Every call a capability gates, made without it. All arguments are zero:
//! the capability check runs before any handler reads one, so the answer is
//! the gate's (EPERM, and a `[CAP-DENY]` line in the kernel log), never an
//! argument error a handler gave first.

use nonos_libc::mk_syscall_raw as raw;

use crate::codes::{GATED, MKIL};
use crate::line::verdict;

const SIGKILL: u64 = 9;
const INIT: u64 = 1;

pub fn authority_escape() {
    for (number, what) in GATED {
        let rc = raw(number, [0, 0, 0, 0, 0, 0]);
        verdict(b"cap-escape", what, rc);
    }
    // MkKill is gated on IPC, which this capsule holds; the handler is what
    // refuses a pid that is neither its child nor its guest.
    let rc = raw(MKIL, [INIT, SIGKILL, 0, 0, 0, 0]);
    verdict(b"cap-escape", b"kill init without ProcessControl, MkKill", rc);
}
