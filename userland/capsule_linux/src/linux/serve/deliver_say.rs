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

//! What delivering a signal says in the log: each handler entered, and each
//! stop that is not served.

/// No guest is ever stopped: job control needs the kernel to hold every
/// thread of a process still, which it does not offer a supervisor.
pub fn stop_unserved(signum: u8) {
    let line = alloc::format!("[LINUX] unserved stop: signal {signum} does not stop a guest\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// SIGURG: Go's runtime sends it to its own threads to preempt a goroutine,
/// dozens of times a run, so a handler entered for it says nothing anyone
/// needs.
const SIGURG: u8 = 23;

/// A handler entered: how a run is going, not anything wrong with it, so a
/// terminal's `linux` command keeps it out of what the person reads.
pub fn say(tid: u32, signum: u8, handler: u64) {
    if signum == SIGURG {
        return;
    }
    let line = alloc::format!("[LINUX] signal {signum} to tid {tid}, handler {handler:#x}\n");
    crate::linux::say::note(line.as_bytes());
}
