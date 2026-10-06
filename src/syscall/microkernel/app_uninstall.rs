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


/*
 * `MkAppUninstall(listing_ptr, listing_len)`: ask for a Linux package the
 * Store installed to be taken away. The personality removes the files its
 * install recorded; MkAppInstallStatus follows it (5 removing, 6 removed).
 * AppInstall, the right to ask for a package, is the right to ask for it
 * gone: neither hosts anything.
 */

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL};

use super::app_install::id;

const HOSTED: &str = "linux.";

pub fn sys_app_uninstall(listing_ptr: u64, listing_len: u64) -> i64 {
    let listing = match id(listing_ptr, listing_len) {
        Ok(Some(s)) => s,
        Ok(None) => return ERRNO_INVAL,
        Err(e) => return e,
    };
    if !listing.strip_prefix(HOSTED).is_some_and(|name| !name.is_empty() && !name.starts_with('.'))
    {
        return ERRNO_INVAL;
    }
    match crate::userspace::init::request_uninstall(listing) {
        true => 0,
        false => ERRNO_BUSY,
    }
}
