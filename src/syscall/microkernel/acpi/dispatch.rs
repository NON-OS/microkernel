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

//! The ACPI capsule's four numbers, routed. Raw arguments rather than the
//! dispatcher's `Args`, which stays private to it.

use super::{
    sys_acpi_cpu, sys_acpi_event, sys_acpi_holds, sys_acpi_pci, sys_acpi_port, sys_acpi_power,
    sys_acpi_region, sys_acpi_sleep, sys_acpi_table, sys_pci_probe_function, sys_smbios_table,
};
use crate::syscall::microkernel::numbers::{
    SYS_ACPI_CPU, SYS_ACPI_EVENT, SYS_ACPI_HOLDS, SYS_ACPI_PCI, SYS_ACPI_PORT, SYS_ACPI_POWER,
    SYS_ACPI_REGION, SYS_ACPI_SLEEP, SYS_ACPI_TABLE, SYS_PCI_PROBE_FUNCTION, SYS_SMBIOS_TABLE,
};

pub fn handle(nr: u64, a0: u64, a1: u64, a2: u64, a3: u64) -> Option<i64> {
    Some(match nr {
        SYS_ACPI_TABLE => sys_acpi_table(a0, a1, a2, a3),
        SYS_ACPI_REGION => sys_acpi_region(a0, a1, a2, a3),
        SYS_ACPI_PORT => sys_acpi_port(a0, a1, a2, a3),
        SYS_ACPI_PCI => sys_acpi_pci(a0, a1, a2, a3),
        SYS_ACPI_POWER => sys_acpi_power(a0),
        SYS_ACPI_EVENT => sys_acpi_event(a0, a1),
        SYS_ACPI_SLEEP => sys_acpi_sleep(a0, a1, a2),
        SYS_SMBIOS_TABLE => sys_smbios_table(a0, a1, a2),
        SYS_ACPI_CPU => sys_acpi_cpu(a0, a1),
        SYS_ACPI_HOLDS => sys_acpi_holds(a0, a1, a2, a3),
        SYS_PCI_PROBE_FUNCTION => sys_pci_probe_function(a0, a1),
        _ => return None,
    })
}
