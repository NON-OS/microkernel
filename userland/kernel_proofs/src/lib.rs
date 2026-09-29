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

pub mod arch;
#[cfg(test)]
pub mod bti_pad;
pub mod bus;
pub mod capabilities;
pub mod elf;
#[cfg(test)]
pub mod fd_fork;
#[cfg(test)]
pub mod firmware_arith;
#[cfg(test)]
pub mod idt_vectors;
#[cfg(test)]
pub mod iommu_window;
#[cfg(test)]
pub mod layout_slots;
pub mod memory;
#[cfg(test)]
pub mod pipe_counts;
#[cfg(test)]
pub mod pci_address;
#[cfg(test)]
pub mod process;
#[cfg(test)]
pub mod procfs_inode;
#[cfg(test)]
pub mod riscv_mmu;
#[cfg(test)]
pub mod range_ends;
#[cfg(test)]
pub mod rsdp_address;
pub mod syscall;
pub mod time;
#[cfg(test)]
pub mod uefi_attrs;
#[cfg(test)]
pub mod uefi_revision;
pub mod security;
pub mod spec;
pub mod sys;
pub mod usercopy;
#[cfg(test)]
pub mod vga_attr;

#[cfg(test)]
mod align_tests;
#[cfg(test)]
mod authorization_tests;
#[cfg(test)]
mod elf_section_tests;
#[cfg(test)]
mod elf_tests;
#[cfg(test)]
mod inbox_name_tests;
#[cfg(test)]
mod permissions_tests;
#[cfg(test)]
mod registry_support;
#[cfg(test)]
mod registry_fold_tests;
#[cfg(test)]
mod registry_set_tests;
#[cfg(test)]
mod syscall_tests;
#[cfg(test)]
mod uefi_cache;
#[cfg(test)]
mod refinement_tests;
#[cfg(test)]
mod usercopy_tests;

#[cfg(kani)]
mod kani_proofs;
