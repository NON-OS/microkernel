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
    sys_acpi_cpu, sys_acpi_device_name, sys_acpi_event, sys_acpi_external_port,
    sys_acpi_function_of, sys_acpi_holds, sys_acpi_hw_error, sys_acpi_pci,
    sys_acpi_platform_device, sys_acpi_port, sys_acpi_power, sys_acpi_proximity, sys_acpi_region,
    sys_acpi_reserved, sys_acpi_sleep, sys_acpi_table, sys_acpi_usb_ports, sys_device_node,
    sys_numa_distance, sys_numa_node_of, sys_pci_probe_function, sys_smbios_table,
    sys_usb_port_facts,
};
use crate::syscall::microkernel::numbers::{
    SYS_ACPI_CPU, SYS_ACPI_DEVICE_NAME, SYS_ACPI_EVENT, SYS_ACPI_EXTERNAL_PORT,
    SYS_ACPI_FUNCTION_OF, SYS_ACPI_HOLDS, SYS_ACPI_HW_ERROR, SYS_ACPI_PCI,
    SYS_ACPI_PLATFORM_DEVICE, SYS_ACPI_PORT, SYS_ACPI_POWER, SYS_ACPI_PROXIMITY, SYS_ACPI_REGION,
    SYS_ACPI_RESERVED, SYS_ACPI_SLEEP, SYS_ACPI_TABLE, SYS_ACPI_USB_PORTS, SYS_DEVICE_NODE,
    SYS_NUMA_DISTANCE, SYS_NUMA_NODE_OF, SYS_PCI_PROBE_FUNCTION, SYS_SMBIOS_TABLE,
    SYS_USB_PORT_FACTS,
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
        SYS_ACPI_FUNCTION_OF => sys_acpi_function_of(a0, a1, a2),
        SYS_ACPI_HW_ERROR => sys_acpi_hw_error(a0),
        SYS_ACPI_PLATFORM_DEVICE => sys_acpi_platform_device(a0, a1),
        SYS_ACPI_DEVICE_NAME => sys_acpi_device_name(a0, a1, a2, a3),
        SYS_ACPI_USB_PORTS => sys_acpi_usb_ports(a0, a1, a2, a3),
        SYS_USB_PORT_FACTS => sys_usb_port_facts(a0, a1, a2),
        SYS_ACPI_EXTERNAL_PORT => sys_acpi_external_port(a0, a1, a2),
        SYS_ACPI_RESERVED => sys_acpi_reserved(a0, a1),
        SYS_ACPI_PROXIMITY => sys_acpi_proximity(a0, a1, a2),
        SYS_NUMA_NODE_OF => sys_numa_node_of(a0, a1),
        SYS_NUMA_DISTANCE => sys_numa_distance(a0, a1),
        SYS_DEVICE_NODE => sys_device_node(a0),
        _ => return None,
    })
}
