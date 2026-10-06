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

//! Host-runnable proofs for kernel isolation and authorization. The real page
//! permission and user-copy bounds source is pulled in via `#[path]` and run
//! directly, so the invariants are proven about the code that actually gates
//! memory access.

extern crate alloc;

#[cfg(test)]
pub mod addr_align;
#[cfg(test)]
pub mod aer_decode;
#[cfg(test)]
pub mod amd_ivhd;
#[cfg(test)]
pub mod amd_vi;
pub mod arch;
#[cfg(test)]
pub mod bti_pad;
pub mod bus;
pub mod capabilities;
#[cfg(test)]
pub mod clock_scale;
#[cfg(test)]
pub mod confine_posture;
pub mod pci_quirk_bits;
#[cfg(test)]
pub mod data_plan;
#[cfg(test)]
pub mod dma_memory_type;
#[cfg(test)]
pub mod dma_placement;
#[cfg(test)]
pub mod dma_pool_bitmap;
#[cfg(test)]
pub mod dma_sync_lines;
#[cfg(test)]
pub mod dmar_scope;
#[cfg(test)]
pub mod boot_screen;
pub mod driver_end;
pub mod device_windows;
pub mod fs;
pub mod hardware;
pub mod fb_frame;
pub mod store_copy_span;
pub mod store_errno;
#[cfg(test)]
pub mod dma_wipe;
#[cfg(test)]
pub mod efi_time;
#[cfg(test)]
pub mod emmc_hosts;
pub mod elf;
#[cfg(test)]
pub mod fd_fork;
#[cfg(test)]
pub mod firmware_arith;
#[cfg(test)]
pub mod firmware_iommu;
#[cfg(test)]
pub mod guest_room;
#[cfg(test)]
pub mod idt_vectors;
#[cfg(test)]
pub mod ioapic_line_mode;
#[cfg(test)]
pub mod smp_bringup;
#[cfg(test)]
pub mod futex_waiters;
#[cfg(test)]
pub mod sched_pick;
#[cfg(test)]
pub mod inbox_budget;
#[cfg(test)]
pub mod iommu_access;
#[cfg(test)]
pub mod iommu_command;
#[cfg(test)]
pub mod iommu_fault;
#[cfg(test)]
pub mod iommu_queue;
#[cfg(test)]
pub mod iommu_remap;
#[cfg(test)]
pub mod iommu_units;
#[cfg(test)]
pub mod iommu_window;
#[cfg(test)]
pub mod iova_space;
#[cfg(test)]
pub mod layout_slots;
pub mod memory;
#[cfg(test)]
pub mod mmio_window;
#[cfg(test)]
pub mod msi_layout;
#[cfg(test)]
pub mod narrowed_args;
#[cfg(test)]
pub mod pci_address;
#[cfg(test)]
pub mod pipe_counts;
#[cfg(test)]
pub mod install_stages;
#[cfg(test)]
pub mod process;
#[cfg(test)]
pub mod procfs_inode;
#[cfg(test)]
pub mod range_ends;
#[cfg(test)]
pub mod reply_share;
#[cfg(test)]
pub mod riscv_mmu;
#[cfg(test)]
pub mod rsdp_address;
#[cfg(test)]
pub mod scan_hidden;
pub mod security;
/// Which registered services take Network to reach.
#[path = "../../../src/services/registry/policy.rs"]
pub mod service_policy;
/// The restart policy init holds a watched service to.
#[cfg(test)]
pub mod lifecycle_state;
/// driver.usb_msc0's search report in words (usb_msc_report_tests).
#[cfg(test)]
#[path = "../../../src/hardware/usb_msc_capsule/report_words.rs"]
pub mod usb_msc_report_words;
#[cfg(test)]
mod usb_msc_report_tests;
/// Which core services init restarts when they end.
#[cfg(test)]
#[path = "../../../src/userspace/init/supervisor/watch_rule.rs"]
pub mod watch_rule;
pub mod spec;
#[cfg(test)]
pub mod surface_unmap;
pub mod sys;
pub mod syscall;
#[cfg(test)]
pub mod thread_refusal;
pub mod time;
#[cfg(test)]
pub mod tls_return;
#[cfg(test)]
pub mod uefi_attrs;
#[cfg(test)]
pub mod uefi_revision;
pub mod usercopy;
#[cfg(test)]
pub mod vga_attr;
#[cfg(test)]
pub mod vmd_domain;

#[cfg(test)]
mod align_tests;
#[cfg(test)]
mod authorization_tests;
#[cfg(test)]
mod elf_section_tests;
#[cfg(test)]
mod elf_tests;
#[cfg(test)]
mod handover_tests;
#[cfg(test)]
mod inbox_name_tests;
#[cfg(test)]
mod ipc_held_tests;
#[cfg(test)]
mod ipc_peers_tests;
#[cfg(test)]
mod service_policy_tests;
#[cfg(test)]
mod supervisor_tests;
#[cfg(test)]
mod permissions_tests;
#[cfg(test)]
mod refinement_tests;
#[cfg(test)]
mod registry_fold_tests;
#[cfg(test)]
mod registry_set_tests;
#[cfg(test)]
mod registry_support;
#[cfg(test)]
mod syscall_tests;
#[cfg(test)]
mod uefi_cache;
#[cfg(test)]
mod usable_span_frame_tests;
#[cfg(test)]
mod usable_span_tests;
#[cfg(test)]
mod user_buffer_tests;
#[cfg(test)]
mod usercopy_tests;

#[cfg(kani)]
mod kani_proofs;
