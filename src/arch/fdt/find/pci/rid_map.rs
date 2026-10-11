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
use crate::arch::fdt::find::smmu::{IommuMap, MapEntry, MAX_MAP_ENTRIES};
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// A requester id map, `iommu-map` or `msi-map` with its mask. Each entry is
/// four cells; a map that ends mid-entry, or names a target `target` refuses,
/// is refused whole.
pub(in crate::arch::fdt::find) fn rid_map<'a>(
    fdt: &Fdt<'a>,
    node: &NodeRef<'a>,
    map_name: &[u8],
    mask_name: &[u8],
    target: impl Fn(&NodeRef<'a>) -> Result<(), FdtError>,
) -> Result<Option<IommuMap>, FdtError> {
    let Some(prop) = fdt.property(node, map_name)? else {
        return Ok(None);
    };
    let mask = match fdt.property(node, mask_name)? {
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
        target(&fdt.by_phandle(entry.phandle)?)?;
        // Past 16 bits a requester id cannot reach; an id past 32 bits
        // cannot be named. Either is a map that cannot be trusted.
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
