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

use super::errnos::{ERRNO_INVAL, ERRNO_PERM};
use crate::process::current_pid;

mod alive;
mod args;
mod thread;

pub use alive::sys_pid_alive;
pub use args::sys_args;
pub use thread::{sys_set_tls, sys_thread_spawn};

pub fn sys_exit(code: i32) -> i64 {
    let Some(pid) = current_pid() else {
        return ERRNO_INVAL;
    };
    // A non-zero code is the one trace a silent capsule leaves: name it.
    if code != 0 {
        use crate::sys::serial::{print, print_dec, println};
        print(b"[EXIT] code=");
        print_dec(code as u64);
        print(b" ");
        let _ = crate::process::with_process(pid, |pcb| print(pcb.name.lock().as_bytes()));
        println(b"");
    }
    crate::process::exit::exit_and_yield(code, false)
}

pub fn sys_yield() -> i64 {
    crate::sched::yield_now();
    0
}

pub fn sys_getpid() -> i64 {
    match current_pid() {
        Some(pid) => pid as i64,
        None => ERRNO_PERM,
    }
}
