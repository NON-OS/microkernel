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
 * How the IO-APIC senses a line a capsule binds: the MADT override when there
 * is one, ISA edge and active-high below GSI 16, PCI level and active-low
 * above. The kernel's pure rule is included by path.
 */

#[path = "../../../../src/arch/x86_64/interrupt/ioapic/ops_route/line_mode.rs"]
pub mod line_mode;
mod tests;
