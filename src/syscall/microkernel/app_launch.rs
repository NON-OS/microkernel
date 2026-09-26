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

//! `MkAppLaunch`: start the program a distribution package installed.

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL};

use super::app_install::id;

/// `MkAppLaunch(listing_ptr, listing_len)`. Queues the run for init; whether
/// the program may start is the exec gate's answer, which checks the trailer
/// the machine minted when it installed the package.
pub fn sys_app_launch(listing_ptr: u64, listing_len: u64) -> i64 {
    let listing = match id(listing_ptr, listing_len) {
        Ok(Some(s)) => s,
        Ok(None) => return ERRNO_INVAL,
        Err(e) => return e,
    };
    let Some(name) = listing.strip_prefix("linux.").filter(|n| !n.is_empty()) else {
        return ERRNO_INVAL;
    };
    match crate::userspace::init::request_run(alloc::string::String::from(name)) {
        true => 0,
        false => ERRNO_BUSY,
    }
}
