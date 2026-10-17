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
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::entry::{Owner, MAX_STREAMS};
use super::owner::Found;

/// A platform device's `iommus`: each an IOMMU phandle and one cell.
pub(super) fn streams(fdt: &Fdt, device: &NodeRef) -> Result<Found, FdtError> {
    let Some(prop) = fdt.property(device, b"iommus")? else {
        return Ok(Err(b"a device with no iommus to key it by"));
    };
    let (mut sids, mut count) = ([0u32; MAX_STREAMS], 0usize);
    for at in (0..prop.data.len()).step_by(8) {
        let unit = fdt.by_phandle(be_u32(prop.data, at)?)?;
        let cells = fdt.property(&unit, b"#iommu-cells")?.map(|c| c.cell_count()).transpose()?;
        if cells != Some(1) || prop.data.len() % 8 != 0 {
            return Ok(Err(b"iommus entries that are not one cell each"));
        }
        let Some(slot) = sids.get_mut(count) else {
            return Ok(Err(b"more stream ids than kept"));
        };
        *slot = be_u32(prop.data, at + 4)?;
        count += 1;
    }
    Ok(Ok(Owner::Streams { sids, count: count as u8 }))
}
