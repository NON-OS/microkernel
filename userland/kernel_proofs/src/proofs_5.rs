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

//! Host proofs, group 5 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

#[path = "acpi_cpu/mod.rs"]
pub mod acpi_cpu;
#[path = "acpi_event/mod.rs"]
pub mod acpi_event;
#[path = "acpi_leave/mod.rs"]
mod acpi_leave;
#[path = "acpi_power/mod.rs"]
pub mod acpi_power;
#[path = "acpi_region/mod.rs"]
pub mod acpi_region;
#[path = "acpi_sleep/mod.rs"]
pub mod acpi_sleep;
#[path = "acpi_tables/mod.rs"]
pub mod acpi_tables;
#[path = "../../../src/arch/aarch64/acpi/mod.rs"]
pub mod arm_acpi;
#[path = "arm_acpi_tests/mod.rs"]
pub mod arm_acpi_tests;
#[path = "boot_cpu_id/mod.rs"]
pub mod boot_cpu_id;
#[path = "device_shown/mod.rs"]
pub mod device_shown;
#[path = "entry_ac_tests.rs"]
mod entry_ac_tests;
#[path = "holder_choice/mod.rs"]
pub mod holder_choice;
#[path = "hyperv_synic/mod.rs"]
pub mod hyperv_synic;
#[path = "hypervisor_detect/mod.rs"]
pub mod hypervisor_detect;
#[path = "irq_flag_tests.rs"]
mod irq_flag_tests;
#[path = "late_probe/mod.rs"]
pub mod late_probe;
#[path = "lpit/mod.rs"]
pub mod lpit;
#[path = "mmap_window/mod.rs"]
pub mod mmap_window;
#[path = "pci_ecam/mod.rs"]
pub mod pci_ecam;
#[path = "registry_firmware_tests.rs"]
mod registry_firmware_tests;
#[path = "registry_namespace_tests.rs"]
mod registry_namespace_tests;
#[path = "seeded_tables/mod.rs"]
pub mod seeded_tables;
#[path = "smbios/mod.rs"]
pub mod smbios;
#[path = "tpm2/mod.rs"]
pub mod tpm2;
#[path = "wdat/mod.rs"]
pub mod wdat;
