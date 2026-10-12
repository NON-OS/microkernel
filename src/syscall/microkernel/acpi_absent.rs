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

//! The ACPI capsule's numbers where there is no ACPI: aarch64 and riscv64
//! boards describe themselves in a device tree. They answer ENOSYS, not the
//! router's -1, so a capsule built for both learns there is nothing here.

use crate::syscall::microkernel::errnos::ERRNO_NOSYS;
use crate::syscall::microkernel::numbers::{
    SYS_ACPI_CPU, SYS_ACPI_DEVICE_NAME, SYS_ACPI_EVENT, SYS_ACPI_EXTERNAL_PORT,
    SYS_ACPI_FUNCTION_OF, SYS_ACPI_HOLDS, SYS_ACPI_HW_ERROR, SYS_ACPI_PCI,
    SYS_ACPI_PLATFORM_DEVICE, SYS_ACPI_PORT, SYS_ACPI_POWER, SYS_ACPI_PROXIMITY, SYS_ACPI_REGION,
    SYS_ACPI_RESERVED, SYS_ACPI_SLEEP, SYS_ACPI_TABLE, SYS_ACPI_USB_PORTS, SYS_DEVICE_NODE,
    SYS_NUMA_DISTANCE, SYS_NUMA_NODE_OF, SYS_PCI_PROBE_FUNCTION, SYS_SMBIOS_TABLE,
    SYS_USB_PORT_FACTS,
};

pub fn handle(nr: u64, a0: u64, a1: u64, a2: u64, a3: u64) -> Option<i64> {
    let _ = (a0, a1, a2, a3);
    matches!(
        nr,
        SYS_ACPI_TABLE
            | SYS_ACPI_REGION
            | SYS_ACPI_PORT
            | SYS_ACPI_PCI
            | SYS_ACPI_POWER
            | SYS_ACPI_EVENT
            | SYS_ACPI_SLEEP
            | SYS_SMBIOS_TABLE
            | SYS_ACPI_CPU
            | SYS_ACPI_HOLDS
            | SYS_PCI_PROBE_FUNCTION
            | SYS_ACPI_FUNCTION_OF
            | SYS_ACPI_HW_ERROR
            | SYS_ACPI_PLATFORM_DEVICE
            | SYS_ACPI_DEVICE_NAME
            | SYS_ACPI_USB_PORTS
            | SYS_USB_PORT_FACTS
            | SYS_ACPI_EXTERNAL_PORT
            | SYS_ACPI_RESERVED
            | SYS_ACPI_PROXIMITY
            | SYS_NUMA_NODE_OF
            | SYS_NUMA_DISTANCE
            | SYS_DEVICE_NODE
    )
    .then_some(ERRNO_NOSYS)
}
