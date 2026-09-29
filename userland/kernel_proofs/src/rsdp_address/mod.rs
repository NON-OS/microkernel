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
 * An RSDP below revision 2 must not hand out its XSDT field, and a revision 2
 * one must pass the full 36-byte checksum.
 *
 * The kernel's multiboot AcpiRsdp is included by path. table_address returned
 * a nonzero XSDT pointer whatever the revision, although below revision 2 the
 * extended checksum that would cover it is never computed, and the extended
 * check passed a revision 2 RSDP with no extended fields and never summed the
 * reserved bytes. The checks below fail against that code.
 */

#[path = "../../../../src/arch/x86_64/multiboot/modules_acpi.rs"]
pub mod modules_acpi;
mod tests;
