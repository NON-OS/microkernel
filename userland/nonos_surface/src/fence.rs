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

//! The release fence: a count per surface, kept by the kernel, that the
//! compositor moves on when it is done reading the surface and the project
//! waits on (the kernel's futex queue, keyed by the surface, not by an
//! address either process could forge).

use nonos_abi::{syscall, N_SURFACE_FENCE_RELEASE, N_SURFACE_FENCE_WAIT};

/// The compositor, holding `handle` attached, is done reading it. Returns
/// the new count, or a negative errno (EPERM when not attached).
pub fn release(handle: u64) -> i64 {
    syscall(N_SURFACE_FENCE_RELEASE, [handle, 0, 0, 0, 0, 0])
}

/// the project of `sid` waits until its release count is not `seen`, at most
/// `timeout_ms` (0 only reads it). Returns the count, or a negative errno.
pub fn wait(sid: u64, seen: u32, timeout_ms: u64) -> i64 {
    syscall(N_SURFACE_FENCE_WAIT, [sid, seen as u64, timeout_ms, 0, 0, 0])
}
