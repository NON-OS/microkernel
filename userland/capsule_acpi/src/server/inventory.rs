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

//! STATUS (1) and DEVICE (2, index): what the capsule has served, and
//! one device of the inventory with its path.

use crate::boot::{path, Acpi};
use alloc::vec::Vec;

pub(super) fn status(acpi: &Acpi) -> Vec<u8> {
    let mut v = Vec::new();
    let routes: usize = acpi.routes.iter().map(|(_, r)| r.len()).sum();
    v.extend_from_slice(&(acpi.devices.len() as u32).to_le_bytes());
    v.extend_from_slice(&(routes as u32).to_le_bytes());
    v.extend_from_slice(&acpi.scis.to_le_bytes());
    v.extend_from_slice(&acpi.gpe_runs.to_le_bytes());
    v.extend_from_slice(&acpi.host.refused.to_le_bytes());
    let armed = acpi.events.as_ref().map_or(0, |e| e.armed.len());
    v.extend_from_slice(&(armed as u32).to_le_bytes());
    v.extend_from_slice(&acpi.aml.rt.dropped_notifications.to_le_bytes());
    v
}

pub(super) fn device(acpi: &Acpi, index: usize) -> Option<Vec<u8>> {
    let d = acpi.devices.get(index)?;
    let mut v = Vec::new();
    v.extend_from_slice(&d.sta.to_le_bytes());
    v.extend_from_slice(&d.adr.unwrap_or(u64::MAX).to_le_bytes());
    v.extend_from_slice(&d.uid.unwrap_or(u64::MAX).to_le_bytes());
    let hid = d.hid.as_ref().map_or(&[][..], |h| h.bytes());
    let p = path(&acpi.aml, d.id);
    for s in [hid, p.as_bytes()] {
        let s = &s[..s.len().min(200)];
        v.push(s.len() as u8);
        v.extend_from_slice(s);
    }
    Some(v)
}
