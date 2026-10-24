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

//! The rest: no service answers, no app store, no battery, no log; a
//! yield or an idle returns at once; a hash or a random draw is refused.

const ENOSYS: i64 = -38;
const ENODEV: i64 = -19;

pub fn mk_yield() -> i64 {
    0
}
pub fn mk_idle_ms(_ms: u64) -> i64 {
    0
}
pub fn mk_ipc_call(_: u64, _: *const u8, _: usize, _: *mut u8, _: usize) -> i64 {
    -110
}
pub fn mk_ipc_send_to_pid(_pid: u32, _buf: *const u8, _len: usize) -> i64 {
    -3
}
pub fn mk_app_install(_listing: &[u8], _release: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_app_install_status(_listing: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_app_uninstall(_listing: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_battery_status() -> i64 {
    ENODEV
}
pub fn mk_log_tail(_out: &mut [u8]) -> i64 {
    0
}
pub fn mk_debug(_buf: *const u8, _len: usize) -> i64 {
    0
}
pub fn crypto_hash(
    _algo: u64,
    _data: *const u8,
    _len: usize,
    _out: *mut u8,
    _out_len: usize,
) -> i64 {
    ENOSYS
}
pub fn mk_spawn_instance(_name: &[u8]) -> i64 {
    ENOSYS
}
pub fn mk_ipc_reply(_dest_pid: u32, _buf: *const u8, _len: usize) -> i64 {
    0
}
pub fn mk_ipc_recv_from(_: u64, _: *mut u8, _: usize, _: u64, _: *mut u32) -> i64 {
    -110
}
pub fn crypto_random(_out: *mut u8, _len: usize) -> i64 {
    ENOSYS
}
