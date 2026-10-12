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
 * The record MkAcpiCpu decodes, and the checks it runs on what the ACPI
 * capsule hands over, included by path as the kernel builds them.
 */

#[path = "../../../../src/hardware/broker/acpi/cpu_layout.rs"]
pub mod cpu_layout;
#[path = "../../../../src/hardware/broker/acpi/cpu_record.rs"]
pub mod cpu_record;
#[path = "../../../../src/hardware/broker/acpi/cpu_take.rs"]
pub mod cpu_take;
#[path = "../../../../src/hardware/broker/acpi/cpu_wire.rs"]
pub mod cpu_wire;

#[cfg(test)]
mod build;
#[cfg(test)]
mod tests;
