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

// A number becomes a pointer only in `mmu::pointer`, which says what each
// kind of address means; anywhere else it is refused at compile time, and
// the few allowed exceptions are counted (nonos-ci/run-static-checks.sh).
#![deny(fuzzy_provenance_casts)]

pub mod abi;
pub mod asm;
pub mod boot;
pub mod boot_entropy;
mod constants;
pub mod context;
pub mod cpu;
pub mod cpu_random;
pub mod exceptions;
pub mod firmware;
pub mod fpu;
pub mod gic;
pub mod interrupt_controller;
pub mod iommu;
pub mod iort;
pub mod mmu;
pub mod pci_intx;
pub mod platform_devices;
pub mod psci;
pub mod rtc;
pub mod security;
pub mod smccc;
pub mod timer;
pub mod uart;

pub use abi::Aarch64;

pub use boot::init;
pub use constants::{PAGE_SIZE, STACK_SIZE};
pub use cpu::{cpu_id, disable_interrupts, enable_interrupts, halt};
pub use exceptions::ExceptionFrame;
pub use gic::{init_gic, send_sgi};
pub use mmu::{init_mmu, map_page, unmap_page, PageTable};
pub use psci::{cpu_off, cpu_on, system_off, system_reset};
pub use timer::{current_time_ns, init_timer, set_timer};
pub use uart::{init_uart, putc, puts};
