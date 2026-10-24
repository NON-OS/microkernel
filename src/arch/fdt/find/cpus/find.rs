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

use crate::arch::fdt::endian::be_cells;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::node::is_cpu;

// Walk `/cpus/cpu@*` and collect each CPU's `reg` (MPIDR Aff bits on
// aarch64, hart id on riscv64). Returns the number of ids written; ids past
// `out.len()` are dropped, which leaves those CPUs parked.
pub fn find(fdt: &Fdt, out: &mut [u64]) -> Result<usize, FdtError> {
    let Some(cpus) = fdt.find_path(b"/cpus")? else {
        return Ok(0);
    };
    let mut filled = 0usize;
    for node in fdt.children(&cpus)? {
        let node = node?;
        if !is_cpu(fdt, &node)? {
            continue;
        }
        let Some(reg) = fdt.property(&node, b"reg")? else {
            continue;
        };
        // The first address of `reg`; a CPU has no size.
        let id = be_cells(reg.data, 0, node.cells.address)?;
        if let Some(slot) = out.get_mut(filled) {
            *slot = id;
            filled += 1;
        }
    }
    Ok(filled)
}
