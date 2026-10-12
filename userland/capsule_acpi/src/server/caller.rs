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

//! Who is asking: the pid that owns a service now, from the registry, and
//! the process a pid's thread belongs to, from the kernel.

use nonos_libc::{mk_service_lookup, mk_syscall_raw};

const THREAD_GROUP: i64 = u32::from_le_bytes(*b"MTGR") as i64;

pub fn owner(name: &[u8]) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}

/// None when the kernel names no process for `pid` (ENOENT) or has no such
/// call; the policy then takes the pid for itself.
pub fn group_of(pid: u32) -> Option<u32> {
    let g = mk_syscall_raw(THREAD_GROUP, [u64::from(pid), 0, 0, 0, 0, 0]);
    u32::try_from(g).ok()
}
