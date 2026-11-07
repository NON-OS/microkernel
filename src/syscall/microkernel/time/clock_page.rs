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

//! Where the caller sees the clock page (`sys::clock::page`), so its libc
//! reads the monotonic clock without a syscall; ENOSYS when the page is not
//! mapped in the caller, and the caller then keeps `MkTimeMonotonic`.

use super::super::errnos::ERRNO_NOSYS;

pub fn sys_clock_page() -> i64 {
    let Some(pid) = crate::process::current_pid() else { return ERRNO_NOSYS };
    match crate::memory::paging::manager::lookup_asid_for_process(pid) {
        Some(asid) if crate::sys::clock::page::mapped_in(asid) => crate::sys::clock::page::VA as i64,
        _ => ERRNO_NOSYS,
    }
}
