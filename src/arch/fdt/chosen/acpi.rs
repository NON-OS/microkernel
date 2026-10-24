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

//! The ACPI RSDP a loader names in `/chosen`, and which of the two
//! descriptions the kernel boots from when it is handed both.
//!
//! The devicetree specification has no property for an RSDP, so this one
//! carries the vendor prefix it asks of a property outside its own set
//! (devicetree specification, property names): `nonos,acpi-rsdp`, the
//! RSDP's physical address. A UEFI loader on an ACPI-only machine passes a
//! tree that holds `/chosen` and little else, as EFI loaders on Arm do.

use super::address::bound;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::gic;
use crate::arch::fdt::parser::Fdt;

/// Which description the kernel boots from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tables {
    DeviceTree,
    /// The ACPI tables from the RSDP at this physical address.
    Acpi(u64),
}

impl<'a> Fdt<'a> {
    /// The RSDP's physical address, if the loader named one. Zero is refused:
    /// it is no table, and taking it would read the vectors page.
    pub fn acpi_rsdp(&self) -> Result<Option<u64>, FdtError> {
        let Some(chosen) = self.find_path(b"/chosen")? else {
            return Ok(None);
        };
        match self.property(&chosen, b"nonos,acpi-rsdp")? {
            None => Ok(None),
            Some(prop) => match bound(&prop)? {
                0 => Err(FdtError::OutOfBounds),
                at => Ok(Some(at)),
            },
        }
    }

    /// The tree whenever it describes the board, which on this arch means it
    /// names the interrupt controller: every board the kernel boots from a
    /// tree has a GIC node, and nothing works without one. ACPI only for a
    /// tree that names an RSDP and no GIC. A tree with neither stays the
    /// tree's, and the GIC step refuses it with its own reason.
    pub fn tables(&self) -> Result<Tables, FdtError> {
        if gic::find(self)?.is_some() {
            return Ok(Tables::DeviceTree);
        }
        Ok(match self.acpi_rsdp()? {
            Some(at) => Tables::Acpi(at),
            None => Tables::DeviceTree,
        })
    }
}
