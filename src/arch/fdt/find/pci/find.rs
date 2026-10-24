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

//! Locating the generic ECAM host bridge and its windows.
//!
//! Only `pci-host-ecam-generic` is taken. Other host bridges carry
//! `device_type = "pci"` too, but their `reg` is a controller's registers,
//! not configuration space: the Pi 4's Broadcom bridge is one, and reading its
//! registers as ECAM would walk garbage as if it were a bus.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::bus_range::bus_range;
use super::info::PciHost;
use super::ranges::{iter as range_iter, SPACE_IO, SPACE_MEM32};

const ECAM: &[&[u8]] = &[b"pci-host-ecam-generic"];

/// The first enabled generic ECAM host bridge, the one `find` describes.
pub fn host<'a>(fdt: &Fdt<'a>) -> Result<Option<NodeRef<'a>>, FdtError> {
    fdt.devices(ECAM).next().transpose().map(|found| found.map(|(node, _)| node))
}

pub fn find(fdt: &Fdt) -> Result<Option<PciHost>, FdtError> {
    let Some(node) = host(fdt)? else {
        return Ok(None);
    };
    let Some((ecam_base, ecam_size)) = fdt.mmio(&node, 0)? else {
        return Ok(None);
    };
    let (bus_start, bus_end) = bus_range(fdt, &node)?;
    let dma_coherent = fdt.property(&node, b"dma-coherent")?.is_some();
    let mut host =
        PciHost { ecam_base, ecam_size, bus_start, bus_end, dma_coherent, ..PciHost::default() };
    let Some(ranges) = fdt.property(&node, b"ranges")? else {
        return Ok(Some(host));
    };
    // The first I/O window and the first 32-bit memory window. Their CPU
    // side is in the bridge's parent's space, so it goes through the same
    // translation as `reg`.
    for range in range_iter(ranges.data, node.cells.address) {
        if range.size == 0 {
            continue;
        }
        if range.space == SPACE_IO && host.io_size == 0 {
            host.io_cpu_base = fdt.translate(&node, range.parent, range.size)?;
            host.io_port_base = range.child;
            host.io_size = range.size;
        } else if range.space == SPACE_MEM32 && host.mem_size == 0 {
            host.mem_cpu_base = fdt.translate(&node, range.parent, range.size)?;
            host.mem_bus_base = range.child;
            host.mem_size = range.size;
        }
    }
    Ok(Some(host))
}
