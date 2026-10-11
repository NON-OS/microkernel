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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::pci::{host, rid_map};
use crate::arch::fdt::find::smmu::{IommuMap, MapEntry, MAX_MAP_ENTRIES};
use crate::arch::fdt::parser::Fdt;

use super::controller::controller;
use super::info::MsiController;

/// The MSI controller the ECAM host bridge's functions write to, and the map
/// from a requester id to the device id it is known by there. `None` when the
/// bridge names none, or names one this kernel does not drive.
///
/// `msi-map` comes first; a bare `msi-parent` means every requester id is
/// its own device id, as Linux reads it. Entries naming two controllers are
/// refused: one function would need both, and the broker binds through one.
pub fn pci_msi_map(fdt: &Fdt) -> Result<Option<(MsiController, IommuMap)>, FdtError> {
    let Some(node) = host(fdt)? else {
        return Ok(None);
    };
    let target = |unit: &_| match fdt.property(unit, b"#msi-cells")? {
        Some(p) if p.cell_count()? > 1 => Err(FdtError::BadCells),
        _ => Ok(()),
    };
    let map = match rid_map(fdt, &node, b"msi-map", b"msi-map-mask", target)? {
        Some(map) => map,
        None => match fdt.property(&node, b"msi-parent")? {
            None => return Ok(None),
            Some(p) if p.data.len() == 4 => every_rid(p.u32()?),
            Some(_) => return Err(FdtError::BadCells),
        },
    };
    let Some(first) = map.entries[..map.count].first() else {
        return Ok(None);
    };
    if map.entries[..map.count].iter().any(|e| e.phandle != first.phandle) {
        return Err(FdtError::BadCells);
    }
    Ok(controller(fdt, first.phandle)?.map(|c| (c, map)))
}

fn every_rid(phandle: u32) -> IommuMap {
    let mut entries = [MapEntry::default(); MAX_MAP_ENTRIES];
    entries[0] = MapEntry { rid_base: 0, phandle, sid_base: 0, length: 0x1_0000 };
    IommuMap { entries, count: 1, mask: u32::MAX }
}
