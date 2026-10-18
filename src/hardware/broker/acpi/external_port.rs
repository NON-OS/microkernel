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

//! A PCIe port the ACPI capsule found declared external-facing in its
//! `_DSD`: the kernel reads the bus range behind it from the port's own
//! bridge registers, never from the capsule, so a capsule cannot flag (or
//! spare) a function the port does not reach.

use super::super::external_range::{bridge_buses, ExternalRange};
use super::super::table::mark_external;
use super::error::RegionError;
use super::pci_access::pci_read;

/// Header type, whose low seven bits say 1 for a PCI-to-PCI bridge.
const HEADER_TYPE: u16 = 0x0E;
/// Primary, secondary and subordinate bus numbers, then latency timer.
const BUS_NUMBERS: u16 = 0x18;

#[derive(Debug)]
pub enum ExternalError {
    /// No bridge, or bus numbers that do not describe one below the port.
    NotBridge,
    Config(RegionError),
}

/// Flags the port and the functions behind it, answering how many.
pub fn declare(
    pid: u32,
    bdf: (u16, u8, u8, u8),
    dma_property: bool,
) -> Result<usize, ExternalError> {
    let header = pci_read(pid, bdf, HEADER_TYPE, 1).map_err(ExternalError::Config)?;
    let buses = pci_read(pid, bdf, BUS_NUMBERS, 4).map_err(ExternalError::Config)?;
    let (secondary, subordinate) =
        bridge_buses(header as u8, buses, bdf.1).ok_or(ExternalError::NotBridge)?;
    let port = (bdf.1, bdf.2, bdf.3);
    let segment = bdf.0;
    Ok(mark_external(ExternalRange { segment, port, secondary, subordinate, dma_property }))
}
