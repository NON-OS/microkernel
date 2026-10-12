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
 * Which devices a DMAR remapping unit covers, from its flags and device
 * scopes, and the rule the broker confines by: a device gets an IOVA only when
 * a unit in service translates it. Also the reserved memory regions (RMRR)
 * firmware keeps doing DMA to, and the ACPI namespace devices (ANDD) with the
 * requester ids their scopes give. The parsers and the rule are included by path.
 */

#[path = "../../../../src/arch/x86_64/acpi/parser/other/acpi_path.rs"]
pub mod acpi_path;
mod andd_tests;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_andd.rs"]
pub mod dmar_andd;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_namespace_scope.rs"]
pub mod dmar_namespace_scope;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_namespace_walk.rs"]
pub mod dmar_namespace_walk;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_rmrr.rs"]
pub mod dmar_rmrr;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_scope_walk.rs"]
pub mod dmar_scope_walk;
mod namespace_tests;
mod rmrr_scope_tests;
mod rmrr_tests;
mod source_tests;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_scope.rs"]
pub mod scope;
mod tests;

/// The name the parsers reach the scope types by in the kernel.
use scope as dmar_scope;
