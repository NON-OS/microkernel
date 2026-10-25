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

//! Overwriting secrets with volatile writes and a fence, so the compiler
//! can neither drop the stores as dead nor move them past the memory's
//! next use.

pub fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/// Overwrite a value that is not bytes, such as a curve scalar or point,
/// with `zero`.
pub(crate) fn wipe_value<T>(v: &mut T, zero: T) {
    // SAFETY: `v` is a valid, aligned, exclusive reference; the old value
    // is overwritten in place without being dropped, which is sound for
    // the plain values this is used on.
    unsafe { core::ptr::write_volatile(v, zero) };
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
