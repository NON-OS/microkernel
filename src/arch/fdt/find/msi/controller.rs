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
use crate::arch::fdt::parser::Fdt;

use super::info::{MsiController, MsiKind};
use super::spis::spi_range;

const KINDS: &[&[u8]] = &[b"arm,gic-v3-its", b"arm,gic-v2m-frame"];

/// The MSI controller at `phandle`, `None` for one this kernel does not drive,
/// such as a host bridge's own, whose devices then have INTx alone. A node
/// that is not an enabled MSI controller, or one whose window cannot hold its
/// registers, is refused.
pub fn controller(fdt: &Fdt, phandle: u32) -> Result<Option<MsiController>, FdtError> {
    let node = fdt.by_phandle(phandle)?;
    if !fdt.is_enabled(&node)? || fdt.property(&node, b"msi-controller")?.is_none() {
        return Err(FdtError::NoSuchNode);
    }
    let Some(kind) = fdt.compatible_with(&node, KINDS)? else {
        return Ok(None);
    };
    let (base, size) = fdt.mmio(&node, 0)?.ok_or(FdtError::BadCells)?;
    let kind = if kind == 0 {
        // GITS_TRANSLATER lives in the second 64 KiB frame, and a device
        // id is the one cell every msi-map entry hands it.
        let one_cell = match fdt.property(&node, b"#msi-cells")? {
            Some(p) => p.cell_count()? == 1,
            None => false,
        };
        if size < 0x2_0000 || !one_cell {
            return Err(FdtError::BadCells);
        }
        MsiKind::Its
    } else {
        if size < 0x1000 {
            return Err(FdtError::BadCells);
        }
        let (spi_base, spi_count) = spi_range(fdt, &node)?;
        MsiKind::V2m { spi_base, spi_count }
    };
    Ok(Some(MsiController { kind, base, size, phandle }))
}
