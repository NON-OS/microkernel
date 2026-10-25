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

//! Which idle states each CPU may enter: its `cpu-idle-states` phandles,
//! resolved against the states kept from `/cpus/idle-states`.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::idle::{CpuStates, IdleState, Refusal};
use crate::arch::fdt::parser::Fdt;

use super::node::is_cpu;

/// For each CPU, in the order `find` lists them, its kept states. A phandle
/// naming a state that was not kept (disabled, not `arm,idle-state`, or
/// missing a value) is passed over: that state is never entered, which is
/// what leaving it out of the table meant. Returns how many CPUs were
/// written; a CPU `find` skips for want of `reg` is skipped here too.
pub fn idle_lists(fdt: &Fdt, table: &[IdleState], out: &mut [CpuStates]) -> Result<usize, Refusal> {
    let Some(cpus) = fdt.find_path(b"/cpus")? else {
        return Ok(0);
    };
    let mut filled = 0usize;
    for node in fdt.children(&cpus)? {
        let node = node?;
        if !is_cpu(fdt, &node)? || fdt.property(&node, b"reg")?.is_none() {
            continue;
        }
        let mut list = CpuStates::default();
        if let Some(prop) = fdt.property(&node, b"cpu-idle-states")? {
            if prop.data.len() % 4 != 0 {
                return Err(FdtError::BadCells.into());
            }
            for at in (0..prop.data.len()).step_by(4) {
                let phandle = be_u32(prop.data, at)?;
                let Some(index) = table.iter().position(|s| s.phandle == phandle) else {
                    continue;
                };
                let slot = list.index.get_mut(usize::from(list.count));
                *slot.ok_or(Refusal::TooManyForCpu)? = index as u8;
                list.count += 1;
            }
        }
        if let Some(slot) = out.get_mut(filled) {
            *slot = list;
            filled += 1;
        }
    }
    Ok(filled)
}
