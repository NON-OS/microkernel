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

mod acpi;
mod core_entry;
pub mod dtb_adapter;
pub mod entry;
pub mod info;
mod init;
mod memory;
pub mod multicore;
mod pci_windows;
mod platform_windows;
mod refuse;
mod relocate;
pub mod stack;
mod start_clock;
#[cfg(any(feature = "nonos-trap-proof-sp0", feature = "nonos-trap-proof-kernel-abort"))]
mod trap_proof;

pub use acpi::acpi_rsdp;
pub use entry::kernel_entry;
pub use info::{BootInfo, MemoryRegion};
pub use init::init;
pub(crate) use memory::init_boot_memory;
pub use multicore::start_secondary_cpus;
pub(crate) use pci_windows::remap as remap_pci_windows;
pub(crate) use refuse::refuse;
pub use stack::setup_stack;
