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

//! One `iommu-addresses` entry's cells.

use crate::arch::fdt::endian::{be_cells, be_u32};
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// The entry at `at`: its device, IOVA and length, in the cells of the
/// device's own bus, and where the next entry starts.
pub(super) fn entry_at<'a>(
    fdt: &Fdt<'a>,
    data: &[u8],
    at: usize,
) -> Result<(NodeRef<'a>, u64, u64, usize), FdtError> {
    let device = fdt.by_phandle(be_u32(data, at)?)?;
    let (a, s) = (device.cells.address as usize, device.cells.size as usize);
    // On a PCI bus: phys.hi, then phys.mid:lo, the IOVA (PCI bus binding,
    // recalled).
    let iova = match a {
        3 => be_cells(data, at + 8, 2)?,
        _ => be_cells(data, at + 4, a as u32)?,
    };
    let len = be_cells(data, at + 4 + 4 * a, s as u32)?;
    Ok((device, iova, len, at + 4 + 4 * (a + s)))
}
