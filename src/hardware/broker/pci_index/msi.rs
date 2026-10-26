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

//! Each PCI function's MSI capability, by broker device id, apart from its
//! handle: only where a device may raise MSI (`msi_only!`). Bind and teardown
//! both read it through `msi_of`, keyed by the handle's id, so they read the
//! same entry; the boot scan installs it whole and a departed function's entry
//! goes with its handle (`late.rs`), so no later handle reads a stale one.

extern crate alloc;

use alloc::vec::Vec;
use spin::RwLock;

use super::PciHandle;
use crate::drivers::pci::types::MsiInfo;

#[path = "msi/table.rs"]
pub(super) mod table;
crate::arch::acpi_only! {
    #[path = "msi/set.rs"]
    pub(super) mod set;
}

pub(super) static MSI: RwLock<Vec<(u64, MsiInfo)>> = RwLock::new(Vec::new());

/// The boot scan's entries, replacing whatever was there.
pub(in crate::hardware::broker) fn install(entries: Vec<(u64, MsiInfo)>) {
    *MSI.write() = entries;
}

/// The MSI capability of the function `handle` names, if it has one.
pub(in crate::hardware::broker) fn msi_of(handle: &PciHandle) -> Option<MsiInfo> {
    table::of(&MSI.read(), handle.device_id)
}
