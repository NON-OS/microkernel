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

//! Host proofs, group 2 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

/// A device's isolation group (K11).
#[path = "acs_group/mod.rs"]
pub mod acs_group;
#[path = "firmware_iommu/mod.rs"]
pub mod firmware_iommu;
#[path = "futex_waiters/mod.rs"]
pub mod futex_waiters;
#[path = "guest_room/mod.rs"]
pub mod guest_room;
#[path = "inbox_budget/mod.rs"]
pub mod inbox_budget;
#[path = "install_stages/mod.rs"]
pub mod install_stages;
#[path = "ioapic_line_mode/mod.rs"]
pub mod ioapic_line_mode;
#[path = "iommu_access/mod.rs"]
pub mod iommu_access;
#[path = "iommu_fault/mod.rs"]
pub mod iommu_fault;
#[path = "iommu_queue/mod.rs"]
pub mod iommu_queue;
#[path = "iommu_remap/mod.rs"]
pub mod iommu_remap;
#[path = "iommu_units/mod.rs"]
pub mod iommu_units;
#[path = "iommu_window/mod.rs"]
pub mod iommu_window;
#[path = "iova_space/mod.rs"]
pub mod iova_space;
#[path = "layout_slots/mod.rs"]
pub mod layout_slots;
#[path = "mmio_window/mod.rs"]
pub mod mmio_window;
#[path = "msi_layout/mod.rs"]
pub mod msi_layout;
#[path = "narrowed_args/mod.rs"]
pub mod narrowed_args;
#[path = "pci_address/mod.rs"]
pub mod pci_address;
#[path = "pipe_counts/mod.rs"]
pub mod pipe_counts;
#[path = "procfs_inode/mod.rs"]
pub mod procfs_inode;
#[path = "range_ends/mod.rs"]
pub mod range_ends;
#[path = "reply_share/mod.rs"]
pub mod reply_share;
#[path = "riscv_mmu/mod.rs"]
pub mod riscv_mmu;
#[path = "rsdp_address/mod.rs"]
pub mod rsdp_address;
#[path = "sched_pick/mod.rs"]
pub mod sched_pick;
#[path = "smp_bringup/mod.rs"]
pub mod smp_bringup;
