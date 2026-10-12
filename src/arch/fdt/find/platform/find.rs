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

use super::describe::describe;
use super::device::PlatformDevice;
use super::kind::{KINDS, NEEDLES};

/// Every enabled, allowlisted node, in tree order. One whose registers,
/// interrupt or DMA cannot be read is left out rather than published at a
/// guess; a capsule handed a wrong window writes to whatever is there.
/// Returns how many were written to `out`.
pub fn find(fdt: &Fdt, out: &mut [PlatformDevice]) -> Result<usize, FdtError> {
    let mut filled = 0usize;
    for found in fdt.devices(NEEDLES) {
        let (node, needle) = found?;
        let Ok(device) = describe(fdt, &node, KINDS[needle]) else {
            continue;
        };
        if let Some(slot) = out.get_mut(filled) {
            *slot = device;
            filled += 1;
        }
    }
    Ok(filled)
}
