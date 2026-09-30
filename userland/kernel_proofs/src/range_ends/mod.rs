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
 * A range must hold the last unit before its true end.
 *
 * The kernel's PortRange, SRAT memory affinity entry and NUMA region are
 * included by path. Each compared an address with a saturated end, so a range
 * reaching the top of its space never held its last port or byte, and two port
 * ranges meeting at 0xFFFF did not overlap, letting reserve_range hand port
 * 0xFFFF out twice. The checks below fail against that code.
 */

#[path = "../../../../src/arch/x86_64/acpi/data/numa.rs"]
pub mod numa;
#[path = "../../../../src/arch/x86_64/port/types/range.rs"]
pub mod range;
#[path = "../../../../src/arch/x86_64/acpi/tables/srat_memory.rs"]
pub mod srat_memory;
mod tests;
