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

//! The ACPI capsule, NUMA and the boot logo.

use crate::syscall::abi::tag4;

/// The ACPI capsule: a checked firmware table, an OperationRegion grant, a port, a config access.
pub const SYS_ACPI_TABLE: u64 = tag4(b"MTBL");
pub const SYS_ACPI_REGION: u64 = tag4(b"MRGN");
pub const SYS_ACPI_PORT: u64 = tag4(b"MRIO");
pub const SYS_ACPI_PCI: u64 = tag4(b"MRPC");
pub const SYS_ACPI_POWER: u64 = tag4(b"MAPW");
pub const SYS_ACPI_EVENT: u64 = tag4(b"MAEV");
pub const SYS_ACPI_SLEEP: u64 = tag4(b"MASL");
pub const SYS_SMBIOS_TABLE: u64 = tag4(b"MSMB");
pub const SYS_ACPI_CPU: u64 = tag4(b"MACP");
pub const SYS_ACPI_HOLDS: u64 = tag4(b"MAHO");
pub const SYS_PCI_PROBE_FUNCTION: u64 = tag4(b"MPPF");
pub const SYS_PCI_DEPART: u64 = tag4(b"MPDP");
pub const SYS_ACPI_FUNCTION_OF: u64 = tag4(b"MAFO");
pub const SYS_ACPI_HW_ERROR: u64 = tag4(b"MAHE");
pub const SYS_ACPI_PLATFORM_DEVICE: u64 = tag4(b"MAPD");
pub const SYS_ACPI_DEVICE_NAME: u64 = tag4(b"MADN");
pub const SYS_ACPI_USB_PORTS: u64 = tag4(b"MAUP");
pub const SYS_USB_PORT_FACTS: u64 = tag4(b"MUPF");
pub const SYS_ACPI_EXTERNAL_PORT: u64 = tag4(b"MAXP");
pub const SYS_ACPI_RESERVED: u64 = tag4(b"MAMR");
pub const SYS_ACPI_PROXIMITY: u64 = tag4(b"MAPX");
pub const SYS_NUMA_NODE_OF: u64 = tag4(b"MNNO");
pub const SYS_NUMA_DISTANCE: u64 = tag4(b"MNDS");
pub const SYS_DEVICE_NODE: u64 = tag4(b"MDNN");
pub const SYS_ACPI_OSC_GRANT: u64 = tag4(b"MAOG");
pub const SYS_NUMA_HMAT: u64 = tag4(b"MNHM");
pub const SYS_ACPI_LPIT: u64 = tag4(b"MALP");
pub const SYS_BOOT_LOGO: u64 = tag4(b"MBGL");
/// The ACPI capsule's first publish pass is done (L55); its owner only.
pub const SYS_ACPI_FIRST_PASS: u64 = tag4(b"MAFP");
/// A native hot-plug slot's four registers and what is below it; acpi owner only.
pub const SYS_ACPI_SLOT: u64 = tag4(b"MRSL");
/// The firmware tables the ACPI capsule was handed, measured; reading attestations.
pub const SYS_ACPI_MEASURED: u64 = tag4(b"MAMT");
/// A table AML loads at run time, measured before it loads; acpi owner only.
pub const SYS_ACPI_LOADED: u64 = tag4(b"MALD");
/// A processor's ceiling under passive cooling, in percent; acpi owner only.
pub const SYS_ACPI_THERMAL_LIMIT: u64 = tag4(b"MATL");
