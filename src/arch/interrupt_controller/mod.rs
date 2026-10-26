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

//! The interrupt controller, as the rest of the kernel needs to see it.
//!
//! Three things are asked of it from shared code: which CPU am I, tell the
//! controller I have finished handling this interrupt, and poke another CPU.
//! x86_64 answers with the local APIC and riscv64 will answer with the CLINT;
//! aarch64 answers with the GIC, whose CPU interface lives in system registers
//! rather than MMIO.
//!
//! Shared code names an [`Ipi`] rather than a vector number, because the number
//! is not portable: x86 IPIs are ordinary interrupt vectors above the device
//! range, while the GIC reserves INTIDs 0 to 15 for exactly this and nothing
//! else. Only the backend knows which number carries which meaning.

#[cfg(feature = "nonos-smp")]
mod adopt_boot_state;
mod cache_boot_id;
// Only where IPIs are delivered (`ipi_dispatch`): riscv64 has none.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod eoi;
mod ipi;
mod ipi_dispatch;
mod kind;
mod local_id;
mod nmi;

#[cfg(feature = "nonos-smp")]
pub(crate) use adopt_boot_state::adopt_boot_state;
pub(crate) use cache_boot_id::{cache_boot_cpu_id, latch_boot_cpu_id};
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub(crate) use eoi::end_of_interrupt;
pub(crate) use ipi::{broadcast_ipi, send_ipi};
/// Called by the IDT builder, which is the only point early enough that every
/// CPU is guaranteed to load a table carrying these vectors.
#[cfg(target_arch = "x86_64")]
pub(crate) use ipi_dispatch::install_gates as install_ipi_gates;
pub(crate) use ipi_dispatch::register_ipi_handlers;
pub use kind::Ipi;
pub(crate) use local_id::local_id;
pub(crate) use nmi::{nmi_one, nmi_others};
