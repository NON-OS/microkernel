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

//! Processes and their streams: nothing runs beside the window on the host,
//! so a spawn is refused and every pid reads as gone.

pub const TTY_STDIN: u64 = 1 << 0;
pub const TTY_STDOUT: u64 = 1 << 1;
pub const TTY_STDERR: u64 = 1 << 2;
pub const TAG_STDOUT: u8 = 1;
pub const TAG_STDERR: u8 = 2;
pub const TAG_BOX: u8 = 3;

const ENOSYS: i64 = -38;
const ESRCH: i64 = -3;

pub fn mk_tool_run(_name: &[u8], _argv: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_kill(_pid: u64, _sig: u64) -> i64 {
    ESRCH
}
pub fn mk_wait(_pid: u64, _timeout_ms: u64) -> i64 {
    ESRCH
}
pub fn mk_pid_alive(_pid: u32) -> bool {
    false
}
pub fn mk_tty_set(_pid: u32, _streams: u64, _cols: u16, _rows: u16) -> i64 {
    ESRCH
}
pub fn mk_proc_output(_pid: u32, _buf: *mut u8, _len: usize) -> i64 {
    ESRCH
}
pub fn mk_proc_input(_pid: u64, _buf: *const u8, _len: u64) -> i64 {
    ESRCH
}
pub fn mk_stdin_read(_buf: *mut u8, _len: u64) -> i64 {
    0
}
