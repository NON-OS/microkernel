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

//! `MkAppInstallStatus`: where an install this caller may ask for stands.
//! A store shows it; asking changes nothing.

use crate::syscall::microkernel::errnos::ERRNO_INVAL;
use crate::userspace::init::{install_stage, Stage};

use super::app_install::id;

/// 0 nothing asked, 1 queued, 2 installing, 3 installed, 4 refused before it
/// started, and 16 plus the installer's reason code when it failed.
pub fn sys_app_install_status(listing_ptr: u64, listing_len: u64) -> i64 {
    let listing = match id(listing_ptr, listing_len) {
        Ok(Some(s)) => s,
        Ok(None) => return ERRNO_INVAL,
        Err(e) => return e,
    };
    match install_stage(&listing) {
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
