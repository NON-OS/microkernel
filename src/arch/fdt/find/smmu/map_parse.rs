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
use crate::arch::fdt::parser::Fdt;

use super::map::IommuMap;

/// The ECAM host bridge's `iommu-map`, or `None` when it has none and its
/// devices reach memory untranslated. Every unit the map names must take a
/// one-cell specifier, the stream id.
pub fn pci_iommu_map(fdt: &Fdt) -> Result<Option<IommuMap>, FdtError> {
    let Some(node) = host(fdt)? else {
        return Ok(None);
    };
    rid_map(fdt, &node, b"iommu-map", b"iommu-map-mask", |unit| {
        match fdt.property(unit, b"#iommu-cells")? {
            Some(p) if p.cell_count()? == 1 => Ok(()),
            _ => Err(FdtError::BadCells),
        }
    })
}
