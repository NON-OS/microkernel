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

//! Caller-side gate: the block surface of driver.usb_msc0 is reached only by
//! callers holding `CAP_DRIVER` or `CAP_STORAGE`, as the other block clients.

use super::error::DriverUsbMscError;
use crate::services::caps::{has_capability, CAP_DRIVER, CAP_STORAGE};

pub(super) fn gate_call() -> Result<u32, DriverUsbMscError> {
    let pid = crate::process::current_pid().ok_or(DriverUsbMscError::NoCallerPid)?;
    if !has_capability(pid, CAP_DRIVER) && !has_capability(pid, CAP_STORAGE) {
        return Err(DriverUsbMscError::AccessDenied);
    }
    Ok(pid)
}
