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


//! BGRT on an Arm machine: the one decoder, BMP check and logo copy,
//! mounted from the x86 parser by path (byte parsers with no x86 in them).
//! Nothing reads BGRT here until the UEFI boot path hands the kernel its
//! ACPI tables; until then MkBootLogo answers ENOSYS on this architecture.

#[path = "../../x86_64/acpi/parser/other/bgrt_copy.rs"]
pub mod bgrt_copy;
#[path = "../../x86_64/acpi/parser/other/bgrt_entry.rs"]
pub mod bgrt_entry;
#[path = "../../x86_64/acpi/parser/other/bmp_check.rs"]
pub mod bmp_check;
