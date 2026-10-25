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

//! Host proofs, group 3 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

#[path = "align_tests.rs"]
mod align_tests;
#[path = "authorization_tests/mod.rs"]
pub(crate) mod authorization_tests;
#[path = "elf_section_tests.rs"]
mod elf_section_tests;
#[path = "elf_tests.rs"]
mod elf_tests;
#[path = "handover_tests.rs"]
mod handover_tests;
#[path = "inbox_name_tests.rs"]
mod inbox_name_tests;
/// An attach whose invalidation fails takes its entry out (K12).
#[path = "iommu_attach/mod.rs"]
pub mod iommu_attach;
#[path = "ipc_held_tests/mod.rs"]
mod ipc_held_tests;
#[path = "ipc_peers_tests.rs"]
mod ipc_peers_tests;
/// The PCI Command register a driver may write (K6).
#[path = "pci_command/mod.rs"]
pub mod pci_command;
#[path = "permissions_tests.rs"]
mod permissions_tests;
#[path = "refinement_tests.rs"]
mod refinement_tests;
#[path = "registry_fold_tests.rs"]
mod registry_fold_tests;
#[path = "registry_set_tests.rs"]
mod registry_set_tests;
#[path = "scan_hidden/mod.rs"]
pub mod scan_hidden;
#[path = "service_policy_tests.rs"]
mod service_policy_tests;
#[path = "supervisor_tests.rs"]
mod supervisor_tests;
#[path = "surface_unmap/mod.rs"]
pub mod surface_unmap;
#[path = "thread_refusal/mod.rs"]
pub mod thread_refusal;
#[path = "tls_return/mod.rs"]
pub mod tls_return;
#[path = "uefi_attrs/mod.rs"]
pub mod uefi_attrs;
#[path = "uefi_revision/mod.rs"]
pub mod uefi_revision;
#[path = "usb_msc_report_tests.rs"]
mod usb_msc_report_tests;
/// The in-kernel virtio-rng's gate: never available while it cannot deliver.
#[path = "vga_attr/mod.rs"]
pub mod vga_attr;
/// Which virtio devices may be confined (K5).
#[path = "virtio_confine/mod.rs"]
pub mod virtio_confine;
#[path = "vmd_domain/mod.rs"]
pub mod vmd_domain;
