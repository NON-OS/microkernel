// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::error::WireError;
use super::table::endpoint;
use super::target::{units, Target};
use crate::services::caps::{has_capability, CAP_DRIVER, CAP_STORAGE};

/// Unit `unit` behind endpoint `index`, for a caller with the authority the
/// other disk clients ask: a driver, or the holder of the store (vfs
/// persisting packages, the data volume's readers).
pub(super) fn gate(index: u8, unit: u16) -> Result<Target, WireError> {
    let pid = crate::process::current_pid().ok_or(WireError::NoCallerPid)?;
    if !has_capability(pid, CAP_DRIVER) && !has_capability(pid, CAP_STORAGE) {
        return Err(WireError::AccessDenied);
    }
    let e = endpoint(index).ok_or(WireError::Dead)?;
    if unit >= units(e) {
        return Err(WireError::NoDevice);
    }
    Ok(Target { e, unit })
}
