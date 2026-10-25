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

/* Which devices a DMAR remapping unit covers, and the broker's rule: a device
 * gets an IOVA only when a unit in service translates it. Also RMRR regions and
 * ANDD namespace devices. The parsers and the rule are included by path. */

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
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_rmrr_names.rs"]
pub mod dmar_rmrr_names;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_scope_walk.rs"]
pub mod dmar_scope_walk;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_walk.rs"]
pub mod dmar_walk;
mod dmar_cut_tests;
mod dmar_hostile;
mod dmar_walk_tests;
mod namespace_tests;
mod rmrr_barred_tests;
mod rmrr_count_tests;
mod rmrr_names_tests;
mod rmrr_own_tests;
mod rmrr_scope_tests;
mod rmrr_tests;
mod source_tests;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_scope.rs"]
pub mod scope;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_parse.rs"]
pub mod dmar_parse;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_cover.rs"]
pub mod dmar_cover;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_translated.rs"]
pub mod dmar_translated;
// The siblings import the scope types by the kernel's file name.
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_ioapic.rs"]
pub mod dmar_ioapic;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_ioapic_source.rs"]
pub mod dmar_ioapic_source;
pub use scope as dmar_scope;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_keep.rs"]
pub mod dmar_keep;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_scope_count.rs"]
pub mod dmar_scope_count;
#[path = "../../../../src/arch/x86_64/acpi/parser/other/dmar_truncated.rs"]
pub mod truncated;
mod cap_tests;
mod ioapic_tests;
mod keep_tests;
mod qemu_igb_tests;
mod tests;
mod truncated_tests;
