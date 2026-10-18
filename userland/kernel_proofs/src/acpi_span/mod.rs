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

//! A table's checksum runs over no more than its own mapped bytes (K11).
//! The real bound is included by path.

#[path = "../../../../src/arch/x86_64/acpi/tables/sdt/entry_count.rs"]
pub mod entry_count;
#[path = "../../../../src/arch/x86_64/acpi/tables/sdt/span.rs"]
pub mod span;
#[cfg(test)]
mod tests;
