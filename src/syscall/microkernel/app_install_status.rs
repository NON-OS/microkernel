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

//! `MkAppInstallStatus`: where an install this caller may ask for stands,
//! and the one line its installer left on why it failed. A store shows
//! them; asking changes nothing.

use crate::syscall::caps::current_caps_or_default;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};
use crate::userspace::init::{install_stage, Stage};

use super::app_install::id;

#[path = "app_install_detail.rs"]
mod detail;

/// `op` 0: 0 nothing asked, 1 queued, 2 installing, 3 installed, 4 refused
/// before it started, and 16 plus the installer's reason code when it
/// failed. `op` 1 and 2 are the failed job's line (app_install_detail.rs).
pub fn sys_app_install_status(
    listing_ptr: u64,
    listing_len: u64,
    op: u64,
    buf_ptr: u64,
    buf_len: u64,
) -> i64 {
    if op == detail::TELL {
        return detail::tell(listing_len, buf_ptr, buf_len);
    }
    if !current_caps_or_default().can_app_install() {
        return ERRNO_PERM;
    }
    let listing = match id(listing_ptr, listing_len) {
        Ok(Some(s)) => s,
        Ok(None) => return ERRNO_INVAL,
        Err(e) => return e,
    };
    match op {
        0 => stage(&listing),
        detail::READ => detail::read(&listing, buf_ptr, buf_len),
        _ => ERRNO_INVAL,
    }
}

fn stage(listing: &str) -> i64 {
    match install_stage(listing) {
        None => 0,
        Some(Stage::Queued) => 1,
        Some(Stage::Running(_)) => 2,
        Some(Stage::Installed) => 3,
        Some(Stage::Refused) => 4,
        Some(Stage::Removing(_)) => 5,
        Some(Stage::Removed) => 6,
        Some(Stage::Failed(code)) => 16 + i64::from(code.clamp(0, 255)),
    }
}
