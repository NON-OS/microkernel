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

//! `MkAppInstall`: ask for a marketplace listing to be installed.

use alloc::string::String;

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL};
use crate::usercopy::{read_user_bytes, validate_user_read};

/// Long enough for any real id, short enough not to become a payload.
const MAX_ID: usize = 96;

/// Only distribution packages have anything to fetch.
const HOSTED: &str = "linux.";

/// `MkAppInstall(listing_ptr, listing_len, release_ptr, release_len)`. An
/// empty release asks for the listing's default. Nothing the caller says
/// about readiness is taken: init asks the market before anything runs.
pub fn sys_app_install(
    listing_ptr: u64,
    listing_len: u64,
    release_ptr: u64,
    release_len: u64,
) -> i64 {
    let listing = match id(listing_ptr, listing_len) {
        Ok(Some(s)) => s,
        Ok(None) => return ERRNO_INVAL,
        Err(e) => return e,
    };
    let release = match id(release_ptr, release_len) {
        Ok(s) => s.unwrap_or_default(),
        Err(e) => return e,
    };
    if !listing.strip_prefix(HOSTED).is_some_and(|name| !name.is_empty() && !name.starts_with('.'))
    {
        return ERRNO_INVAL;
    }
    match crate::userspace::init::request_install(listing, release) {
        true => 0,
        false => ERRNO_BUSY,
    }
}

/// One id argument. `None` for an empty one. The id reaches a URL and a store
/// path, so it is held to what a package id actually is.
pub(super) fn id(ptr: u64, len: u64) -> Result<Option<String>, i64> {
    let len = usize::try_from(len).map_err(|_| ERRNO_INVAL)?;
    if len == 0 {
        return Ok(None);
    }
    if len > MAX_ID || validate_user_read(ptr, len).is_err() {
        return Err(ERRNO_INVAL);
    }
    let raw = read_user_bytes(ptr, len).map_err(|_| ERRNO_FAULT)?;
    let allowed =
        |b: &u8| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'+' | b'.' | b'@');
    if !raw.iter().all(allowed) {
        return Err(ERRNO_INVAL);
    }
    String::from_utf8(raw).map(Some).map_err(|_| ERRNO_INVAL)
}
