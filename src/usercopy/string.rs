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

//! Read a NUL-terminated user string.

use super::error::UsercopyError;
use super::policy::{check_range, USER_SPACE_END};
use super::string_scan::scan_until_nul;
use crate::arch::run_without_interrupts;

const MAX_STRING_LEN: usize = 4096;

pub fn read_user_string(
    user_ptr: u64,
    max_len: usize,
) -> Result<alloc::string::String, UsercopyError> {
    // Bounded by what is left of the user half as well as by the ceiling.
    let room = USER_SPACE_END.saturating_sub(user_ptr).saturating_add(1);
    let safe_len = max_len.min(MAX_STRING_LEN).min(room as usize);
    if check_range(user_ptr, safe_len)?.is_none() {
        return Ok(alloc::string::String::new());
    }
    let mut buf = alloc::vec![0u8; safe_len];
    let actual_len = run_without_interrupts(|| scan_until_nul(user_ptr, safe_len, &mut buf))?;
    buf.truncate(actual_len);
    alloc::string::String::from_utf8(buf).map_err(|_| UsercopyError::InvalidUtf8)
}
