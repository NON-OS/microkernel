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

/*
 * SMP bring-up and local APIC arithmetic, checked against the Intel SDM
 * (vol. 3: MP initialisation, APIC, x2APIC) and against MADT shapes real
 * firmware ships: hybrid parts with sparse ids, duplicate Local APIC and
 * x2APIC entries, placeholders, ids past 0xFE, more CPUs than MAX_CPUS.
 *
 * Each file is the kernel's own, included by path. They were written pure
 * for this: no hardware access, no crate paths.
 */

#[path = "../../../../src/arch/x86_64/interrupt/apic/constants.rs"]
pub mod apic_constants;
#[path = "../../../../src/arch/x86_64/interrupt/apic/plan.rs"]
pub mod apic_plan;
#[path = "../../../../src/smp/boot_claim.rs"]
pub mod boot_claim;
#[path = "../../../../src/smp/topology/core_kind.rs"]
pub mod core_kind;
#[path = "../../../../src/smp/topology/detection/probe_x86_64/plan.rs"]
pub mod madt_plan;
// Only the timer band is checked; the TSC band and the cache are kernel-side.
#[allow(dead_code)]
#[path = "../../../../src/sys/apic/local_calibrate/consts.rs"]
pub mod calibrate_consts;
/* Mounted once, in idt_vectors; a second mount is clippy's duplicate_mod. */
pub use crate::idt_vectors::vectors as idt_vectors;
mod core_kind_tests;
mod tests;
