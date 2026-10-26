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

use super::describe;
use crate::arch::aarch64::boot::acpi::record;
use crate::arch::fdt::chosen::Tables;
use crate::arch::fdt::Fdt;
use crate::sys::serial::Line;

/// The RSDP the loader named, kept whichever description the kernel boots
/// from: the arch-neutral table readers want it on a
/// tree boot too. Runs first, because the bare tree an ACPI boot is handed
/// has no memory nodes and would otherwise be refused for those, which says
/// nothing useful.
///
/// An ACPI-only machine is refused by name. Booting it on the tree's path
/// would drive QEMU virt's addresses on a server; this kernel does not read
/// MADT, GTDT or SPCR on aarch64 yet.
pub(super) fn populate(fdt: &Fdt) -> Result<(), &'static [u8]> {
    record(fdt.acpi_rsdp().map_err(describe)?);
    match fdt.tables().map_err(describe)? {
        Tables::DeviceTree => Ok(()),
        Tables::Acpi(at) => {
            Line::new().str(b"[NONOS] ACPI-only machine, RSDP at ").hex(at).end();
            Err(b"ACPI tables and no device tree: aarch64 does not read ACPI yet")
        }
    }
}
