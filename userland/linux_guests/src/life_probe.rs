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

//! Whether a guest can make a child and hear back from it.
//!
//! Not an isolation property: a shell forks for every command it runs, so a
//! personality whose children never run cannot host one. The wait is
//! bounded, so a child that never runs is reported instead of hanging here.

use crate::sys::{call, out, NANOSLEEP};

const FORK: u64 = 57;
const WAIT4: u64 = 61;
const EXIT_GROUP: u64 = 231;
const WNOHANG: u64 = 1;
const CHILD_STATUS: u64 = 7;
const WAIT_SECS: u32 = 10;

pub fn run() {
    let pid = call(FORK, [0; 6]);
    if pid == 0 {
        let _ = call(EXIT_GROUP, [CHILD_STATUS, 0, 0, 0, 0, 0]);
    }
    if pid < 0 {
        out(format!("[GUEST] life fork failed: errno={}\n", -pid).as_bytes());
        return;
    }
    let mut status = 0i32;
    for _ in 0..WAIT_SECS * 10 {
        let rc = call(WAIT4, [pid as u64, &mut status as *mut i32 as u64, WNOHANG, 0, 0, 0]);
        if rc == pid {
            let code = (status >> 8) & 0xff;
            out(format!("[GUEST] life fork: child {pid} exited {code}\n").as_bytes());
            return;
        }
        let tenth = [0u64, 100_000_000];
        let _ = call(NANOSLEEP, [tenth.as_ptr() as u64, 0, 0, 0, 0, 0]);
    }
    out(format!("[GUEST] life fork: child {pid} did not finish in {WAIT_SECS}s\n").as_bytes());
}
