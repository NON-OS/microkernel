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

//! A surface's release fence (nonos_surface::fence): a count the kernel
//! keeps per surface. The compositor moves it on once it is done reading a
//! buffer; the project waits for it to move.

use crate::syscall::{call_raw, N_MK_SURFACE_FENCE_RELEASE, N_MK_SURFACE_FENCE_WAIT};

/// The caller, which holds `handle` shared to it, is done reading it.
/// Returns the surface's new count, or a negative errno.
#[no_mangle]
pub extern "C" fn mk_surface_fence_release(handle: u64) -> i64 {
    call_raw(N_MK_SURFACE_FENCE_RELEASE, [handle, 0, 0, 0, 0, 0])
}

/// the project of `sid` waits until its count is not `seen`, at most
/// `timeout_ms` (0 only reads it). Returns the count, or a negative errno.
#[no_mangle]
pub extern "C" fn mk_surface_fence_wait(sid: u64, seen: u32, timeout_ms: u64) -> i64 {
    call_raw(N_MK_SURFACE_FENCE_WAIT, [sid, seen as u64, timeout_ms, 0, 0, 0])
}
