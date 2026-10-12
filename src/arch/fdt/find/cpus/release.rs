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

//! Where each CPU waits to be released, when firmware parks it on a spin
//! table rather than behind PSCI.
//!
//! A CPU whose `enable-method` is "spin-table" polls the 64-bit location its
//! `cpu-release-addr` names and jumps to whatever non-zero address appears
//! there (Devicetree Specification v0.4, 3.8.1 and 5.4.2). The property is
//! always two cells, whatever `/cpus` says its `#address-cells` is.

use crate::arch::fdt::endian::be_u64;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::node::is_cpu;

/// For each CPU, in the order `find` lists them, the release address when it
/// waits on a spin table, `None` when something else (PSCI) starts it.
/// Returns how many entries were written; a CPU `find` would have skipped
/// for want of `reg` is skipped here too, so the indices line up.
pub fn release_addresses(fdt: &Fdt, out: &mut [Option<u64>]) -> Result<usize, FdtError> {
    let Some(cpus) = fdt.find_path(b"/cpus")? else {
        return Ok(0);
    };
    let mut filled = 0usize;
    for node in fdt.children(&cpus)? {
        let node = node?;
        if !is_cpu(fdt, &node)? || fdt.property(&node, b"reg")?.is_none() {
            continue;
        }
        let spin = match fdt.property(&node, b"enable-method")? {
            Some(method) => method.stringlist().any(|m| m == b"spin-table"),
            None => false,
        };
        let address = match fdt.property(&node, b"cpu-release-addr")? {
            Some(addr) if spin => {
                if addr.data.len() != 8 {
                    return Err(FdtError::BadCells);
                }
                Some(be_u64(addr.data, 0)?)
            }
            _ => None,
        };
        if let Some(slot) = out.get_mut(filled) {
            *slot = address;
            filled += 1;
        }
    }
    Ok(filled)
}
