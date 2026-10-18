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

use super::limits::MAX_STREAMS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Iommus {
    /// No `iommus`: nothing in the tree says a unit translates its DMA.
    None,
    /// The unit, by phandle, and the device's stream ids on it.
    Streams { unit: u32, sids: [u32; MAX_STREAMS], count: u8 },
    /// An `iommus` the kernel cannot honour, and why.
    Unreadable(&'static [u8]),
}

/// `device`'s `iommus`. A phandle naming no node refuses the read; entries
/// the kernel cannot honour (cells other than one, two units, more streams
/// than kept) are `Unreadable`, never cut short.
pub fn iommus(fdt: &Fdt, device: &NodeRef) -> Result<Iommus, FdtError> {
    let Some(prop) = fdt.property(device, b"iommus")? else {
        return Ok(Iommus::None);
    };
    if prop.data.is_empty() || prop.data.len() % 8 != 0 {
        return Ok(Iommus::Unreadable(b"iommus entries that are not one cell each"));
    }
    let (mut sids, mut count, mut unit) = ([0u32; MAX_STREAMS], 0usize, None);
    for at in (0..prop.data.len()).step_by(8) {
        let phandle = be_u32(prop.data, at)?;
        let node = fdt.by_phandle(phandle)?;
        let cells = fdt.property(&node, b"#iommu-cells")?.map(|c| c.cell_count()).transpose()?;
        if cells != Some(1) {
            return Ok(Iommus::Unreadable(b"iommus entries that are not one cell each"));
        }
        if *unit.get_or_insert(phandle) != phandle {
            return Ok(Iommus::Unreadable(b"iommus naming more than one IOMMU"));
        }
        let Some(slot) = sids.get_mut(count) else {
            return Ok(Iommus::Unreadable(b"more stream ids than kept"));
        };
        *slot = be_u32(prop.data, at + 4)?;
        count += 1;
    }
    Ok(Iommus::Streams { unit: unit.unwrap_or(0), sids, count: count as u8 })
}
