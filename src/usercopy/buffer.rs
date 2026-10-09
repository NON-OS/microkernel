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

//! The kernel buffer a copy of a caller-chosen length lands in, with no kernel
//! dependencies so a host proof can run it.

use alloc::vec::Vec;

use super::error::UsercopyError;

/// An empty vector with room for `len` bytes, or `SizeTooLarge` when the heap
/// cannot give that much. The kernel's allocation failure handler halts the
/// machine, so a length a caller chose, megabytes of it, must never reach an
/// allocation that cannot fail.
pub(crate) fn take_buffer(len: usize) -> Result<Vec<u8>, UsercopyError> {
    let mut buf = Vec::new();
    buf.try_reserve_exact(len).map_err(|_| UsercopyError::SizeTooLarge)?;
    Ok(buf)
}
