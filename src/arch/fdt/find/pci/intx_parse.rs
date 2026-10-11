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
use crate::arch::fdt::parser::Fdt;

use super::bus_range::bus_range;
use super::find::host;
use super::intx::{IntxMap, IntxRoute, MAX_INTX};
use super::intx_entry::entry;

/// The ECAM host's `interrupt-map`, or `None` when it has none and INTx is
/// not wired at all. Every entry must name a GIC SPI, level-high or rising
/// edge (the only SPI triggers a GIC has); anything else refuses the map,
/// since one wrong entry routes a device's interrupt to another's line.
pub fn interrupt_map(fdt: &Fdt) -> Result<Option<IntxMap>, FdtError> {
    let Some(node) = host(fdt)? else { return Ok(None) };
    let Some(map) = fdt.property(&node, b"interrupt-map")? else { return Ok(None) };
    let mask = fdt.property(&node, b"interrupt-map-mask")?.ok_or(FdtError::BadCells)?;
    let int_cells = fdt.property(&node, b"#interrupt-cells")?.ok_or(FdtError::BadCells)?;
    if int_cells.cell_count()? != 1
        || mask.data.len() != 16
        || be_u32(mask.data, 4)? != 0
        || be_u32(mask.data, 8)? != 0
    {
        return Err(FdtError::BadCells);
    }
    let mut out = IntxMap {
        routes: [IntxRoute::default(); MAX_INTX],
        count: 0,
        mask_hi: be_u32(mask.data, 0)?,
        mask_pin: be_u32(mask.data, 12)?,
        root_bus: bus_range(fdt, &node)?.0,
    };
    let mut at = 0;
    while at < map.data.len() {
        let (route, used) = entry(fdt, &map.data[at..])?;
        if out.count == MAX_INTX {
            return Err(FdtError::BadCells);
        }
        out.routes[out.count] = IntxRoute {
            child_hi: route.child_hi & out.mask_hi,
            pin: route.pin & out.mask_pin,
            ..route
        };
        out.count += 1;
        at += used;
    }
    Ok(Some(out))
}
