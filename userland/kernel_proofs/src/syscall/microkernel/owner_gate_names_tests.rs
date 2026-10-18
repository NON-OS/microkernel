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

//! F-95: the capsule's calls, the twelve that once took the Admin bit alone
//! among them, are the owner's; the shared ones are not.

use super::owner_gate::owner_only;
use super::owner_gate_tests::tag_of;

#[test]
fn the_capsules_calls_are_the_owners() {
    let once_admin_alone = [
        "SYS_ACPI_TABLE",
        "SYS_ACPI_REGION",
        "SYS_ACPI_PORT",
        "SYS_ACPI_PCI",
        "SYS_ACPI_EVENT",
        "SYS_ACPI_SLEEP",
        "SYS_SMBIOS_TABLE",
        "SYS_ACPI_CPU",
        "SYS_ACPI_HOLDS",
        "SYS_ACPI_FUNCTION_OF",
        "SYS_ACPI_HW_ERROR",
        "SYS_ACPI_POWER",
    ];
    for name in once_admin_alone.iter().chain(&["SYS_ACPI_SLOT", "SYS_ACPI_LOADED"]) {
        assert!(owner_only(u64::from(u32::from_le_bytes(tag_of(name)))), "{name}");
    }
    for name in ["SYS_ACPI_DEVICE_NAME", "SYS_ACPI_MEASURED", "SYS_BOOT_LOGO", "SYS_NUMA_NODE_OF"] {
        assert!(!owner_only(u64::from(u32::from_le_bytes(tag_of(name)))), "{name}");
    }
    assert!(!owner_only(0) && !owner_only(u64::MAX));
}
