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

//! `MkAcpiRegion(space, base, length, out_ptr) -> 0`: grant the ACPI capsule
//! one OperationRegion window, if the broker's rule allows it. SystemMemory
//! (space 0, ACPI 6.5 section 19.6.100) is mapped and `out_ptr` receives the
//! address of `base` and the MMIO grant id; SystemIO (space 1) is recorded
//! for `MkAcpiPort`. Other spaces are not granted through this call.

use core::mem::size_of;

use super::errno::region_errno;
use crate::hardware::broker::acpi::{grant_io, grant_memory};
use crate::hardware::broker::unmap_grant;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_NOTSUP, ERRNO_PERM};
use crate::usercopy::{validate_user_write, write_user_value};

const SYSTEM_MEMORY: u64 = 0;
const SYSTEM_IO: u64 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
struct RegionOut {
    user_va: u64,
    grant_id: u64,
}

pub fn sys_acpi_region(space: u64, base: u64, length: u64, out_ptr: u64) -> i64 {
    let Some(pid) = current_pid() else { return ERRNO_PERM };
    match space {
        SYSTEM_IO => grant_io(pid, base, length).map_or_else(region_errno, |()| 0),
        SYSTEM_MEMORY => {
            if out_ptr == 0 || validate_user_write(out_ptr, size_of::<RegionOut>()).is_err() {
                return ERRNO_FAULT;
            }
            let g = match grant_memory(pid, base, length) {
                Ok(g) => g,
                Err(e) => return region_errno(e),
            };
            let out = RegionOut { user_va: g.user_va, grant_id: g.grant_id };
            if write_user_value(out_ptr, &out).is_err() {
                // The capsule cannot learn where the window is; take it back.
                let _ = unmap_grant(pid, g.grant_id);
                return ERRNO_FAULT;
            }
            0
        }
        _ => ERRNO_NOTSUP,
    }
}
