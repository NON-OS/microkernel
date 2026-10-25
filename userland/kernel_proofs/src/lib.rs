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

//! Host-runnable proofs for kernel isolation and authorization, run on the
//! kernel's own source pulled in by `#[path]`; test groups in proofs_*.rs.

extern crate alloc;

pub mod arch;
pub mod bus;
pub mod capabilities;
pub mod device_windows;
#[cfg(test)]
pub mod dmar_scope;
pub mod driver_end;
/// The driver-grant surface (A10): each call's cap, hold and grant.
pub mod driver_grants;
pub mod elf;
pub mod fb_frame;
pub mod fs;
pub mod hardware;
#[cfg(test)]
pub mod idt_vectors;
#[cfg(test)]
pub mod iommu_command;
pub mod memory;
pub mod pci_quirk_bits;
#[cfg(test)]
pub mod process;
pub mod security;
/// Which registered services take Network to reach.
#[path = "../../../src/services/registry/policy.rs"]
pub mod service_policy;
pub mod services;
pub mod store_copy_span;
pub mod store_errno;
#[cfg(kani)]
mod kani_proofs;
/// The test groups, named at the root with what they share (proofs.rs).
#[cfg(test)]
mod proofs;
#[cfg(test)]
pub use proofs::*;
#[cfg(test)]
mod registry_support;
mod mmap_masked;
pub mod spec;
pub mod sys;
pub mod syscall;
pub mod time;
pub mod usercopy;
