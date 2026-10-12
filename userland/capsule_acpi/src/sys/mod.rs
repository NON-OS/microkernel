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

//! The capsule's calls into the kernel.

pub mod acpi;
pub mod acpi_cpu;
pub mod acpi_event;
pub mod acpi_holds;
pub mod acpi_pci;
pub mod acpi_power;
pub mod acpi_sleep;
pub mod clock;
pub mod device_name;
pub mod function_of;
pub mod hw_error;
pub mod log;
pub mod pci_probe;
pub mod platform;
pub mod ports;
pub mod smbios;
