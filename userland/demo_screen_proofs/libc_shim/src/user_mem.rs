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

//! The shim's side of a call, standing for the kernel's: the caller's pointers
//! arrive as addresses, as the real libc hands them over (`call_raw` passes
//! each as a u64), and are read and written here.

/// Write `v` at `addr`.
///
/// # Safety
/// `addr` is a writable `T` of the caller's, as the real call requires.
pub(crate) unsafe fn put<T>(addr: usize, v: T) {
    // SAFETY: the caller's guarantee above.
    unsafe { core::ptr::write_unaligned(addr as *mut T, v) }
}

/// The `len` bytes at `addr`.
///
/// # Safety
/// `addr` is `len` readable bytes of the caller's, held for `'a`.
pub(crate) unsafe fn bytes<'a>(addr: usize, len: usize) -> &'a [u8] {
    // SAFETY: the caller's guarantee above.
    unsafe { core::slice::from_raw_parts(addr as *const u8, len) }
}
