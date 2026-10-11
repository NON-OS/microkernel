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
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// The SPIs a v2m frame's node says it raises, `(0, 0)` when it says
/// nothing. Some frames report a wrong range in `MSI_TYPER` and the node
/// corrects it, so both properties or neither: one alone is a range half
/// stated, and a range reaching past the last SPI would hand a device lines
/// the GIC cannot deliver.
pub(super) fn spi_range(fdt: &Fdt, node: &NodeRef) -> Result<(u32, u32), FdtError> {
    let cell = |name: &[u8]| -> Result<Option<u32>, FdtError> {
        match fdt.property(node, name)? {
            Some(p) if p.data.len() == 4 => p.u32().map(Some),
            Some(_) => Err(FdtError::BadCells),
            None => Ok(None),
        }
    };
    match (cell(b"arm,msi-base-spi")?, cell(b"arm,msi-num-spis")?) {
        (None, None) => Ok((0, 0)),
        (Some(base), Some(count)) if spi_range_ok(base, count) => Ok((base, count)),
        _ => Err(FdtError::BadCells),
    }
}

/// SPIs are INTIDs 32 to 1019.
pub(super) const fn spi_range_ok(base: u32, count: u32) -> bool {
    base >= 32 && count != 0 && count <= 1020 - base
}
