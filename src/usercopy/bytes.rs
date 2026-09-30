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

//! Heap-allocated byte buffers around the directmap byte path.
//! `copy.rs` is the canonical entry; this layer is a convenience for
//! callers that want a `Vec<u8>` instead of a fixed slice.

use super::copy::{copy_from_user, copy_to_user};
use super::error::UsercopyError;

const MAX_USER_COPY_SIZE: usize = 16 * 1024 * 1024;

pub fn read_user_bytes(user_ptr: u64, len: usize) -> Result<alloc::vec::Vec<u8>, UsercopyError> {
    if len > MAX_USER_COPY_SIZE {
        return Err(UsercopyError::SizeTooLarge);
    }
    if len == 0 {
        copy_from_user(user_ptr, &mut [])?;
        return Ok(alloc::vec::Vec::new());
    }
    /*
     * Zeroed and filled one serve unit at a time, answering TLB shootdowns
     * in between: at the ceiling this is 16 MiB of masked work, which under
     * emulation outlasts a peer's patience. Each piece is walked afresh, so
     * nothing translated is held across the serve point.
     */
    let mut buf = alloc::vec::Vec::with_capacity(len);
    while buf.len() < len {
        let at = buf.len();
        if at != 0 {
            crate::smp::serve_shootdowns();
        }
        let from = user_ptr.checked_add(at as u64).ok_or(UsercopyError::AddressOverflow)?;
        buf.resize(at + (len - at).min(crate::smp::SERVE_UNIT), 0);
        copy_from_user(from, &mut buf[at..])?;
    }
    Ok(buf)
}

pub fn write_user_bytes(user_ptr: u64, data: &[u8]) -> Result<(), UsercopyError> {
    copy_to_user(user_ptr, data)
}
