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

//! SystemIO windows: granted once, then every access made by the kernel on
//! the capsule's behalf, with the rule asked again each time so a port a
//! driver claimed after the grant is no longer the capsule's to touch.

use super::error::RegionError;
use super::granting::GRANTING;
use super::kernel::records::{self, Space};
use super::kernel::Machine;
use super::report;
use super::rule;

pub fn grant_io(pid: u32, base: u64, len: u64) -> Result<(), RegionError> {
    let _granting = GRANTING.lock();
    rule::io(base, len, &Machine { pid }).map_err(|r| {
        report::refused(Space::Io, base, len, r);
        RegionError::Refused(r)
    })?;
    let end = base + len;
    if records::holds(pid, Space::Io, base, end) {
        return Ok(());
    }
    if !records::insert(pid, Space::Io, base, end) {
        return Err(RegionError::Full);
    }
    report::granted(Space::Io, base, len);
    Ok(())
}

/// The port, once the caller holds it and the rule still allows it.
pub(super) fn port(pid: u32, port: u64, width: u8) -> Result<u16, RegionError> {
    if !matches!(width, 1 | 2 | 4) || port % u64::from(width) != 0 {
        return Err(RegionError::BadWidth);
    }
    let end = port.checked_add(u64::from(width)).ok_or(RegionError::NotGranted)?;
    if !records::holds(pid, Space::Io, port, end) {
        return Err(RegionError::NotGranted);
    }
    rule::io(port, u64::from(width), &Machine { pid }).map_err(RegionError::Refused)?;
    u16::try_from(port).map_err(|_| RegionError::NotGranted)
}
