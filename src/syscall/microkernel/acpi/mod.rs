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

//! The ACPI capsule's calls: its firmware tables and the OperationRegion
//! windows the broker's rule grants it. x86_64 only, as ACPI is here.

mod asker;
mod cpu;
mod device_name;
mod dispatch;
mod errno;
mod event;
mod event_act;
mod external;
mod function_of;
mod holds;
mod hw_error;
mod numa;
mod pci;
mod platform;
mod port;
mod power;
mod probe;
mod region;
mod reserved;
mod sleep;
mod smbios;
mod table;
mod usb_ports;

pub use cpu::sys_acpi_cpu;
pub use device_name::sys_acpi_device_name;
pub use dispatch::handle;
pub use event::sys_acpi_event;
pub use external::sys_acpi_external_port;
pub use function_of::sys_acpi_function_of;
pub use holds::sys_acpi_holds;
pub use hw_error::sys_acpi_hw_error;
pub use numa::{sys_acpi_proximity, sys_numa_distance, sys_numa_node_of};
pub use pci::sys_acpi_pci;
pub use platform::sys_acpi_platform_device;
pub use port::sys_acpi_port;
pub use power::sys_acpi_power;
pub use probe::sys_pci_probe_function;
pub use region::sys_acpi_region;
pub use reserved::sys_acpi_reserved;
pub use sleep::sys_acpi_sleep;
pub use smbios::sys_smbios_table;
pub use table::sys_acpi_table;
pub use usb_ports::{sys_acpi_usb_ports, sys_usb_port_facts};
