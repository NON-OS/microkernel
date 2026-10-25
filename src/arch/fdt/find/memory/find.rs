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

use super::range::MemoryRange;

// Every top-level memory node's `reg`, translated like any other: a node is
// memory by `device_type = "memory"`, or by name on trees old enough to omit
// it. Ranges past `out.len()` are dropped, which loses RAM but never claims
// any that is not there. Empty ranges, as an unfilled Pi template carries,
// are skipped.
pub fn find(fdt: &Fdt, out: &mut [MemoryRange]) -> Result<usize, FdtError> {
    let mut filled = 0usize;
    for node in fdt.nodes() {
        let node = node?;
        if node.depth != 1 || !is_memory(fdt, &node)? || !fdt.is_enabled(&node)? {
            continue;
        }
        let mut index = 0usize;
        while let Some((base, size)) = fdt.mmio(&node, index)? {
            index += 1;
            if size == 0 {
                continue;
            }
            if let Some(slot) = out.get_mut(filled) {
                *slot = MemoryRange { base, size };
                filled += 1;
            }
        }
    }
    Ok(filled)
}

fn is_memory(fdt: &Fdt, node: &crate::arch::fdt::NodeRef) -> Result<bool, FdtError> {
    match fdt.property(node, b"device_type")? {
        Some(kind) => Ok(kind.string() == Ok(b"memory")),
        None => Ok(node.base_name() == b"memory"),
    }
}
