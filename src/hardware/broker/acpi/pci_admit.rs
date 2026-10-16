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

//! Whether the ACPI capsule may make one PCI_Config access, and the first
//! access to each function recorded as its grant and logged once.

use super::error::RegionError;
use super::granting::GRANTING;
use super::kernel::records::{self, Space};
use super::kernel::Machine;
use super::report;
use super::rule;
use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::types::PciAddress;

/// (segment, bus, device, function) packed as the log line prints it.
pub(super) fn packed(bdf: (u16, u8, u8, u8)) -> u64 {
    (u64::from(bdf.0) << 24) | (u64::from(bdf.1) << 16) | (u64::from(bdf.2) << 8) | u64::from(bdf.3)
}

pub(super) fn admit(
    pid: u32,
    bdf: (u16, u8, u8, u8),
    at: u16,
    width: u8,
    write: bool,
) -> Result<ConfigSpace, RegionError> {
    if bdf.0 != 0 {
        return Err(RegionError::Unsupported);
    }
    if !matches!(width, 1 | 2 | 4) || at % u16::from(width) != 0 {
        return Err(RegionError::BadWidth);
    }
    let key = packed(bdf);
    let _granting = GRANTING.lock();
    rule::pci_config(bdf, write, &Machine { pid }).map_err(|r| {
        report::refused(Space::Pci, key, 1, r);
        RegionError::Refused(r)
    })?;
    if !records::holds(pid, Space::Pci, key, key + 1) {
        if !records::insert(pid, Space::Pci, key, key + 1) {
            return Err(RegionError::Full);
        }
        report::granted(Space::Pci, key, 1);
    }
    Ok(ConfigSpace::new(PciAddress::new(bdf.1, bdf.2, bdf.3)))
}
