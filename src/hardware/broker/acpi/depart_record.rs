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

//! An ACPI platform record that has left: a controller firmware describes
//! by `_HID` behind a dock, gone with it. It leaves as a PCI function does
//! (`depart`): everything its holder was given is revoked first, its MMIO
//! windows fenced, and its record retired with its kept name. Only records
//! the ACPI capsule published (bus kind ACPI) may be named this way.

use super::super::claim::lookup;
use super::super::device::BusKind;
use super::super::table;
use super::depart::{revoke, DepartError, Departed};
use super::platform_paths::forget;

/// `eject`: firmware is about to be asked to remove it, which waits for
/// its holder; otherwise it is already gone, and the holder loses it.
pub fn depart_record(id: u64, eject: bool) -> Result<Departed, DepartError> {
    let acpi = table::list().iter().any(|r| r.device_id == id && r.bus_kind == BusKind::Acpi as u8);
    if !acpi {
        return Err(DepartError::Unknown);
    }
    let holder = lookup(id).map(|c| c.pid);
    if let (true, Some(pid)) = (eject, holder) {
        return Err(DepartError::Held(pid));
    }
    let counts = holder.map(|pid| revoke(pid, id));
    let _ = table::remove(id);
    forget(id);
    super::probe_say::left(None, id, holder.zip(counts));
    Ok(Departed { device_id: id, holder })
}
