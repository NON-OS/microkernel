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
 * IVHD device entries: which requester ids an IVRS table's AMD IOMMUs
 * cover, included by path and fed synthetic tables laid out per AMD IOMMU
 * spec 48882, 5.2.2.2, and hostile bytes.
 */

#[path = "../../../../src/arch/x86_64/acpi/parser/other/ivhd_entry.rs"]
pub mod ivhd_entry;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/ivhd_kind.rs"]
pub mod ivhd_kind;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/ivhd_scope.rs"]
pub mod ivhd_scope;
mod fixture;
mod hostile_tests;
mod tests;
