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

//! Taking the blob the firmware left in memory.

use core::slice;

use super::error::FdtError;
use super::header::{self, HEADER_SIZE};
use super::limits::MAX_BLOB;
use super::parser::Fdt;

impl Fdt<'static> {
    /// The tree at `ptr`.
    ///
    /// # Safety
    /// `ptr` must be mapped and readable for the header's 40 bytes, and for
    /// `totalsize` bytes once those name a size no larger than `MAX_BLOB`, and
    /// nothing may write there while the result lives.
    pub unsafe fn from_ptr(ptr: *const u8) -> Result<Self, FdtError> {
        if ptr.is_null() {
            return Err(FdtError::NullPointer);
        }
        if !(ptr as usize).is_multiple_of(8) {
            return Err(FdtError::Misaligned);
        }
        // SAFETY: the caller vouches for the first HEADER_SIZE bytes.
        let head = unsafe { slice::from_raw_parts(ptr, HEADER_SIZE) };
        let total = header::parse(head)?.totalsize as usize;
        // Cap before widening the slice: a corrupt header must not turn into
        // a slice over memory nobody mapped.
        if total > MAX_BLOB {
            return Err(FdtError::TooLarge);
        }
        // SAFETY: `total` is the size the header names, within MAX_BLOB, which
        // the caller vouches for.
        let blob = unsafe { slice::from_raw_parts(ptr, total) };
        Fdt::from_bytes(blob)
    }
}
