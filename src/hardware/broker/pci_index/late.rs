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

//! The ACPI broker's changes to the index: a function added after the boot
//! scan, and one that departed.

use super::msi::{set::set, MSI};
use super::{PciHandle, INDEX};
use crate::drivers::pci::types::MsiInfo;

/// One function added after the boot scan (`table::add_pci_function`), with
/// its MSI capability if it has one.
pub(in crate::hardware::broker) fn push(handle: PciHandle, msi: Option<MsiInfo>) {
    set(&mut MSI.write(), handle.device_id, msi);
    INDEX.write().push(handle);
}

/// A function that left (`acpi::depart`): its handle goes with it, and its
/// MSI entry, so a later arrival at the same address is read afresh.
pub(in crate::hardware::broker) fn take(device_id: u64) -> Option<PciHandle> {
    set::<MsiInfo>(&mut MSI.write(), device_id, None);
    let mut index = INDEX.write();
    let at = index.iter().position(|h| h.device_id == device_id)?;
    Some(index.remove(at))
}
