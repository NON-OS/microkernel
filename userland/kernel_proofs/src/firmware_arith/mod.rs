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
 * Arithmetic on firmware-supplied values must not overflow.
 *
 * The kernel's UEFI memory descriptor, per-CPU region, I/O APIC entry, and
 * MMIO and port statistics snapshots are included by path. Each added or
 * multiplied values that firmware or a wrapping counter controls with an
 * overflow-checked operator, which aborts the kernel, or in this release build
 * wraps. The checks below fail against that code.
 */

#[path = "../../../../src/arch/x86_64/acpi/data/ioapic.rs"]
pub mod ioapic;
#[path = "../../../../src/arch/x86_64/uefi/tables/memory_desc.rs"]
pub mod memory_desc;
#[path = "../../../../src/memory/layout/types/percpu.rs"]
pub mod percpu;
#[path = "../../../../src/memory/mmio/types/stats_snapshot.rs"]
pub mod stats_snapshot;
#[path = "../../../../src/arch/x86_64/port/stats_snapshot.rs"]
pub mod port_snapshot;
mod tests;
