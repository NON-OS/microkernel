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
use crate::arch::fdt::parser::Fdt;

use super::info::PciHost;
use super::ranges::{iter as range_iter, SPACE_IO};

const ECAM: &[&[u8]] = &[b"pci-host-ecam-generic"];

pub fn find(fdt: &Fdt) -> Result<Option<PciHost>, FdtError> {
    let Some(found) = fdt.devices(ECAM).next() else {
        return Ok(None);
    };
    let (node, _) = found?;
    let Some((ecam_base, ecam_size)) = fdt.mmio(&node, 0)? else {
        return Ok(None);
    };
    let mut host = PciHost { ecam_base, ecam_size, ..PciHost::default() };
    if let Some(ranges) = fdt.property(&node, b"ranges")? {
        // A bridge may list several memory windows but conventionally has at
        // most one I/O window. Its CPU side is in the bridge's parent's
        // space, so it goes through the same translation as `reg`.
        for range in range_iter(ranges.data, node.cells.address) {
            if range.space == SPACE_IO && range.size != 0 {
                host.io_cpu_base = fdt.translate(&node, range.parent, range.size)?;
                host.io_port_base = range.child;
                host.io_size = range.size;
                break;
            }
        }
    }
    Ok(Some(host))
}
