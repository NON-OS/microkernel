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

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::pci::host;
use crate::arch::fdt::parser::Fdt;

use super::map::{IommuMap, MapEntry, MAX_MAP_ENTRIES};

/// The ECAM host bridge's `iommu-map`, or `None` when it has none and its
/// devices reach memory untranslated. Each entry is four cells, since every
/// unit the map names must take a one-cell specifier, the stream id; a unit
/// that takes more, or a map that ends mid-entry, is refused.
pub fn pci_iommu_map(fdt: &Fdt) -> Result<Option<IommuMap>, FdtError> {
    let Some(node) = host(fdt)? else {
        return Ok(None);
    };
    let Some(prop) = fdt.property(&node, b"iommu-map")? else {
        return Ok(None);
    };
    let mask = match fdt.property(&node, b"iommu-map-mask")? {
        Some(p) if p.data.len() == 4 => p.u32()?,
        Some(_) => return Err(FdtError::BadCells),
        None => u32::MAX,
    };
    if prop.data.len() % 16 != 0 || prop.data.len() / 16 > MAX_MAP_ENTRIES {
        return Err(FdtError::BadCells);
    }
    let mut map = IommuMap { entries: [MapEntry::default(); MAX_MAP_ENTRIES], count: 0, mask };
    for chunk in prop.data.chunks_exact(16) {
        let cell = |i: usize| be_u32(chunk, i * 4);
        let entry = MapEntry {
            rid_base: cell(0)?,
            phandle: cell(1)?,
            sid_base: cell(2)?,
            length: cell(3)?,
        };
        let unit = fdt.by_phandle(entry.phandle)?;
        match fdt.property(&unit, b"#iommu-cells")? {
            Some(p) if p.cell_count()? == 1 => {}
            _ => return Err(FdtError::BadCells),
        }
        // Past 16 bits a requester id cannot reach; a stream id past 32
        // bits cannot be named. Either is a map that cannot be trusted.
        let rid_end = u64::from(entry.rid_base) + u64::from(entry.length);
        let sid_end = u64::from(entry.sid_base) + u64::from(entry.length);
        if rid_end > 0x1_0000 || sid_end > 1 << 32 {
            return Err(FdtError::BadCells);
        }
        map.entries[map.count] = entry;
        map.count += 1;
    }
    Ok(Some(map))
}
