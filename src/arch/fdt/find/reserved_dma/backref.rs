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

//! Whether a device names a `/reserved-memory` child back through its own
//! `memory-region` (the maintainers' ruling): an `iommu-addresses` entry the
//! device does not name back is not the device's range, and is not mapped.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// The child's phandle, if it has one: a child without one cannot be named.
pub(super) fn phandle(fdt: &Fdt, child: &NodeRef) -> Result<Option<u32>, FdtError> {
    for name in [&b"phandle"[..], b"linux,phandle"] {
        if let Some(p) = fdt.property(child, name)? {
            return p.u32().map(Some);
        }
    }
    Ok(None)
}

/// True when `device`'s `memory-region`, one phandle per cell, lists
/// `child`. A list that is not whole cells is damaged and refused.
pub(super) fn names_back(
    fdt: &Fdt,
    device: &NodeRef,
    child: Option<u32>,
) -> Result<bool, FdtError> {
    let (Some(child), Some(list)) = (child, fdt.property(device, b"memory-region")?) else {
        return Ok(false);
    };
    if list.data.len() % 4 != 0 {
        return Err(FdtError::BadCells);
    }
    for at in (0..list.data.len()).step_by(4) {
        if be_u32(list.data, at)? == child {
            return Ok(true);
        }
    }
    Ok(false)
}
