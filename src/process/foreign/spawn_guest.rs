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

//! Making a guest: a process, a kernel stack, a place in its family's seal.

use alloc::format;

use crate::kernel_core::process_spawn::allocate_kernel_stack;
use crate::process::core::types::Priority;
use crate::process::core::{create_process_with_parent, ProcessState};
use crate::syscall::microkernel::errnos::{ERRNO_AGAIN, ERRNO_EXIST, ERRNO_NOMEM, ERRNO_PERM};

/// The one place a guest comes into being; one it cannot record, it ends.
pub(in crate::process::foreign) fn empty_guest(supervisor: u32, name: &[u8]) -> Result<u32, i64> {
    if !super::room::has_room(super::registry::guest_count(supervisor)) {
        return Err(ERRNO_AGAIN);
    }
    // A family the kernel could not seal whole does not start.
    super::seal::reserve(supervisor)?;
    let tag = format!("foreign:{}", core::str::from_utf8(name).unwrap_or("guest"));
    let pid = create_process_with_parent(&tag, ProcessState::New, Priority::Normal, 0, None)
        .map_err(|_| ERRNO_NOMEM)?;
    // A guest that cannot be finished is ended, as one that cannot be
    // recorded is below: left in the table it held its pid for good.
    if allocate_kernel_stack(pid).is_err() {
        return Err(unborn(pid, ERRNO_NOMEM));
    }
    // Born with the ambient set, core exec, IPC and memory; a guest holds none.
    if crate::process::caps::install_spawn(pid, 0).is_none() {
        return Err(unborn(pid, ERRNO_PERM));
    }
    // Read back rather than assumed, so the log states what the guest holds.
    crate::sys::serial::print(b"[FOREIGN] guest pid=");
    crate::sys::serial::print_hex(pid as u64);
    crate::sys::serial::print(b" caps=");
    crate::sys::serial::print_hex(crate::process::caps::bits(pid).unwrap_or(u64::MAX));
    crate::sys::serial::println(b"");
    if !super::enrol::enrol(pid, supervisor) {
        return Err(unborn(pid, ERRNO_EXIST));
    }
    Ok(pid)
}

/// End a guest that was created but cannot run, and pass on why.
fn unborn(pid: u32, errno: i64) -> i64 {
    crate::process::exit::teardown(pid, errno as i32, false);
    errno
}
