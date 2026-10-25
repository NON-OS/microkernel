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

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod abi;
mod acpi_capsule;
mod active_page_table_root;
pub mod cache_flush;
mod cache_line_ops;
mod cache_span;
pub(crate) mod common;
pub mod console;
pub mod context;
pub mod cpu;
pub mod cpu_random;
mod dma_snoops_caches;
mod early_screen;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
pub mod fdt;
mod firmware_entropy;
pub mod firmware_table;
pub mod halt;
mod hv;
pub(crate) mod idle;
pub mod image_holes;
mod init_boot_memory;
mod init_broker_irq_routing;
pub mod interrupt_controller;
mod iommu_unit;
#[cfg(not(target_arch = "x86_64"))]
mod iommu_window;
#[cfg(target_arch = "x86_64")]
pub mod nonos_boot;
mod outgoing;
pub mod paging;
mod pci_intx_source;
mod percpu_base;
pub mod port_io;
pub(crate) mod power;
mod publish_described_devices;
mod remap_pci_windows;
mod report_dma_posture;
mod reserved_regions;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
mod rmrr_regions;
mod run_without_interrupts;
mod stack_pointer;
mod time_counter;
mod tpm_window;
pub mod trap;
pub mod user_access;
pub mod wall_clock;
pub mod write_combining;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
pub use abi::ArchOps;
pub(crate) use acpi_capsule::{acpi_capsule_exit, acpi_first_pass_wait, acpi_from_init};
pub(crate) use active_page_table_root::active_page_table_root;
pub(crate) use common::own_core_kind::own_core_kind;
#[cfg(target_arch = "x86_64")]
pub use cpu::init_cpu_features;
pub use cpu::{cpu_yield, disable_interrupts, enable_interrupts, get_cpu_id, idle_cpu};
pub(crate) use dma_snoops_caches::{
    dma_snoops_caches, pci_dma_coherent, uncached_grants_need_uncached_frames,
};
pub(crate) use early_screen::early_screen;
pub(crate) use firmware_entropy::firmware_entropy;
pub use halt::halt_loop;
pub use hv::*;
pub use image_holes::deliberately_unmapped;
pub(crate) use init_boot_memory::init_boot_memory;
pub(crate) use init_broker_irq_routing::init_broker_irq_routing;
pub(crate) use iommu_unit::iommu_unit_in_service;
#[cfg(not(target_arch = "x86_64"))]
pub(crate) use iommu_window::iommu_register_window;
pub(crate) use outgoing::keep_outgoing_user_state;
pub(crate) use pci_intx_source::pci_intx_source;
pub(crate) use percpu_base::set as set_percpu_base;
pub(crate) use publish_described_devices::publish_described_devices;
pub(crate) use remap_pci_windows::remap_pci_windows;
pub(crate) use report_dma_posture::report_dma_posture;
pub(crate) use reserved_regions::{device_reserved_regions, reserved_memory_regions};
pub(crate) use rmrr_regions::device_rmrr_regions;
pub(crate) use run_without_interrupts::run_without_interrupts;
pub(crate) use stack_pointer::stack_pointer;
pub(crate) use time_counter::{read_time_counter, time_counter_hz};
pub(crate) use tpm_window::tpm_window;
#[cfg(target_arch = "x86_64")]
pub type Arch = x86_64::abi::X86_64;
#[cfg(target_arch = "aarch64")]
pub type Arch = aarch64::abi::Aarch64;
#[cfg(target_arch = "riscv64")]
pub type Arch = riscv64::abi::Riscv64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(target_arch = "x86_64")]
pub use nonos_boot as boot;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
