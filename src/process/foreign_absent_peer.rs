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

//! The peer calls, which reach into a guest's memory: with no guest hosted,
//! each refuses.

use crate::syscall::microkernel::errnos::ERRNO_NOSYS;

pub fn sys_peer_map(_pid: u64, _addr: u64, _len: u64, _prot: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_peer_copy(_pid: u64, _guest_addr: u64, _buf: u64, _len: u64, _to_guest: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_peer_protect(_pid: u64, _addr: u64, _len: u64, _prot: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_peer_tls(_pid: u64, _base: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_peer_unmap(_pid: u64, _addr: u64, _len: u64) -> i64 {
    ERRNO_NOSYS
}
