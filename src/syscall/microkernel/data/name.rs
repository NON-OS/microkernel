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

//! A data file's name, copied in from the caller and held to one shape: a
//! slash, then 1 to 63 of [A-Za-z0-9._-]. No directories, no dot names.

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL};

pub(super) const NAME_MAX: usize = 64;

pub(super) fn copy_name(ptr: u64, len: u64) -> Result<([u8; NAME_MAX], usize), i64> {
    let len = len as usize;
    if ptr == 0 || !(2..=NAME_MAX).contains(&len) {
        return Err(ERRNO_INVAL);
    }
    let mut out = [0u8; NAME_MAX];
    if crate::usercopy::copy_from_user(ptr, &mut out[..len]).is_err() {
        return Err(ERRNO_FAULT);
    }
    let ok = |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-');
    if out[0] != b'/' || out[1] == b'.' || !out[1..len].iter().all(ok) {
        return Err(ERRNO_INVAL);
    }
    Ok((out, len))
}
