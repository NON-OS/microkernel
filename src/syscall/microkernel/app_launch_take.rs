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

//! The word an MkAppLaunch gives, read from its caller and admitted
//! (app_launch_word.rs): none, or one a capsule SERVES pairs with the
//! listing may give, never one from a free launch.

use alloc::string::String;
use alloc::vec::Vec;

use crate::services::registry::caller_owns;
use crate::services::registry::serves::{LISTINGS, WORDS};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};
use crate::usercopy::{read_user_bytes, validate_user_read};

use super::word::{admit_word, MOST};

/// The word `[ptr, ptr + len)` gives a launch of `listing`, `free` whether
/// AppInstall or the shell asks; EINVAL for one it may not give.
pub(super) fn taken(free: bool, listing: &str, ptr: u64, len: u64) -> Result<Option<String>, i64> {
    let len = usize::try_from(len).map_err(|_| ERRNO_INVAL)?;
    if len == 0 {
        return Ok(None);
    }
    if free || len > MOST || validate_user_read(ptr, len).is_err() {
        return Err(ERRNO_INVAL);
    }
    let raw: Vec<u8> = read_user_bytes(ptr, len).map_err(|_| ERRNO_FAULT)?;
    match admit_word(caller_owns, listing, &raw, (LISTINGS, WORDS)) {
        Ok(word) => Ok(word.map(String::from)),
        Err(()) => Err(ERRNO_INVAL),
    }
}
