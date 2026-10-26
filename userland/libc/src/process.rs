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

use crate::syscall::{
    call_raw, N_MK_ARGS, N_MK_CAPS_OF, N_MK_GETPID, N_MK_KILL, N_MK_PID_ALIVE, N_MK_WAIT,
};

/// The rights a SpawnBroker holder may ask `mk_caps_of` about.
pub const CAPS_OF_SPAWN_WINDOW: u64 = 1 << 24;
pub const CAPS_OF_APP_INSTALL: u64 = 1 << 30;

/// For a SpawnBroker holder: whether `pid`, the sender of a message queued at
/// `sent_ms` (`mk_ipc_recv_from_at`), holds `bit`. 1, 0 or a negative errno:
/// EPERM to a non-broker, EINVAL for another bit, ESRCH for no such sender.
pub extern "C" fn mk_caps_of(pid: u32, bit: u64, sent_ms: u64) -> i64 {
    call_raw(N_MK_CAPS_OF, [pid as u64, bit, sent_ms, 0, 0, 0])
}

pub extern "C" fn mk_pid_alive(pid: u32) -> bool {
    call_raw(N_MK_PID_ALIVE, [pid as u64, 0, 0, 0, 0, 0]) == 1
}

pub extern "C" fn mk_getpid() -> u32 {
    let rc = call_raw(N_MK_GETPID, [0, 0, 0, 0, 0, 0]);
    if rc < 0 {
        0
    } else {
        rc as u32
    }
}

/// The most bytes an argv joins to, the kernel's ARGV_BYTES_MAX (pinned equal
/// by scripts/check_argv_bound.py): a buffer this size holds any argv.
pub const ARGV_BYTES_MAX: usize = 4096;

pub extern "C" fn mk_args(buf: *mut u8, len: usize) -> i64 {
    call_raw(N_MK_ARGS, [buf as u64, len as u64, 0, 0, 0, 0])
}

pub extern "C" fn mk_wait(pid: u64, timeout_ms: u64) -> i64 {
    call_raw(N_MK_WAIT, [pid, timeout_ms, 0, 0, 0, 0])
}

pub extern "C" fn mk_kill(pid: u64, sig: u64) -> i64 {
    call_raw(N_MK_KILL, [pid, sig, 0, 0, 0, 0])
}
