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

//! A platform device the rule admitted, added to the device table with the
//! record `platform_record` builds, its `_HID`, `_UID` and path kept beside
//! it. No DMAR ANDD or IVRS ACPI-HID entry is parsed yet, so its DMA is not
//! confined by an IOMMU domain, and the line says so.

use super::super::acpi_i2c::clock::source_clock_hz;
use super::super::table;
use super::platform_paths::{keep, Named};
use super::platform_record::record;
use super::platform_wire::{Platform, I2C};
use crate::sys::serial::Line;

pub fn add(p: Platform) -> u64 {
    let clock = if p.class == I2C { source_clock_hz(&p.hid) } else { 0 };
    let id = table::register_platform_device(record(&p, clock));
    let name = p.hid.iter().copied().take_while(|c| *c != 0).collect::<alloc::vec::Vec<u8>>();
    let mut l = Line::new();
    l.str(b"[ACPI] platform ").str(&p.path).str(b" ").str(&name).str(b" device=").hex(id);
    l.str(b" windows=")
        .dec(p.windows.len() as u64)
        .str(b"; no ANDD/ACPI-HID entry parsed: unconfined");
    l.end();
    keep(Named { device_id: id, hid: p.hid, uid: p.uid, path: p.path });
    id
}
