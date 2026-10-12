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

//! SRAT and SLIT on an Arm machine: the one SRAT decoder and SLIT reader,
//! mounted from the x86 parser by path (they are byte parsers with no x86
//! in them). GICC affinity entries are this architecture's processors.

#[path = "../../x86_64/acpi/parser/other/slit_matrix.rs"]
pub mod slit_matrix;
#[path = "../../x86_64/acpi/parser/other/srat_decode.rs"]
mod srat_decode;
#[path = "../../x86_64/acpi/parser/other/srat_entry.rs"]
pub mod srat_entry;
