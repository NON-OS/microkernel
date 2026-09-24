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

//! Typed accessors over the directmap byte path.

use core::mem::{align_of, size_of, MaybeUninit};

use super::copy::{copy_from_user, copy_to_user};
use super::error::UsercopyError;

pub fn read_user_value<T: Copy>(user_ptr: u64) -> Result<T, UsercopyError> {
    if misaligned::<T>(user_ptr) {
        return Err(UsercopyError::MisalignedAddress);
    }
    // Zeroed as bytes, not as a `T`.
    let mut value = MaybeUninit::<T>::uninit();
    // SAFETY: eK@nonos.systems - the storage is exactly this many
    // bytes and holds no value yet, so writing over it drops nothing.
    unsafe { core::ptr::write_bytes(value.as_mut_ptr() as *mut u8, 0, size_of::<T>()) };
    /*
     * SAFETY: eK@nonos.systems - the slice covers exactly the bytes
     * of one `T`, at a pointer this frame owns, over storage the
     * write above initialised.
     */
    let dst =
        unsafe { core::slice::from_raw_parts_mut(value.as_mut_ptr() as *mut u8, size_of::<T>()) };
    copy_from_user(user_ptr, dst)?;
    // SAFETY: eK@nonos.systems - the copy filled every byte or
    // returned, and `value_rules` binds what a `T` here may be.
    Ok(unsafe { value.assume_init() })
}

pub fn write_user_value<T: Copy>(user_ptr: u64, value: &T) -> Result<(), UsercopyError> {
    if misaligned::<T>(user_ptr) {
        return Err(UsercopyError::MisalignedAddress);
    }
    // SAFETY: eK@nonos.systems - the slice covers exactly the bytes of
    // the caller's `T` and is only read.
    let src =
        unsafe { core::slice::from_raw_parts(value as *const T as *const u8, size_of::<T>()) };
    copy_to_user(user_ptr, src)
}

fn misaligned<T>(user_ptr: u64) -> bool {
    let align = align_of::<T>();
    align > 1 && (user_ptr as usize) % align != 0
}
