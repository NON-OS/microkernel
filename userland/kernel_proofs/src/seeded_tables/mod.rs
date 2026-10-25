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

//! F-98: seeded hostile tables over the kernel's pure ACPI parsers (MADT
//! on both architectures, SRAT, HMAT, SLIT, MCFG, HPET, FADT, SPCR, DBG2,
//! PCCT, BGRT and its BMP, and the run-time loaded-table check): every one
//! loads or is refused, never panics or indexes out, and never holds more
//! than its cap. The x86 decoders are mounted by path as the kernel builds
//! them.

#[path = "../../../../src/arch/x86_64/acpi/parser/madt/walk.rs"]
mod madt_walk;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/mcfg_decode.rs"]
mod mcfg_decode;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/hpet_decode.rs"]
mod hpet_decode;
#[path = "../../../../src/arch/x86_64/acpi/hw/gas.rs"]
#[allow(dead_code)] // the FADT's registers, written only by `leave_events` here
pub(crate) mod gas;
#[path = "../../../../src/arch/x86_64/acpi/hw/fadt_decode.rs"]
#[allow(dead_code)]
pub(crate) mod fadt_decode;
// F-107's exit writes, beside the two files they build on (`acpi_leave`).
#[path = "../../../../src/arch/x86_64/acpi/hw/leave_events.rs"]
pub(crate) mod leave_events;
#[path = "../../../../src/arch/x86_64/acpi/hw/sleep.rs"]
#[allow(dead_code)]
pub(crate) mod sleep;

mod gen;
mod hmat_gen;
mod rng;
mod sweep;

mod arm_tests;
mod debug_tests;
mod fixed_tests;
mod hmat_tests;
mod image_tests;
mod mcfg_tests;
mod numa_tests;
mod x86_tests;
