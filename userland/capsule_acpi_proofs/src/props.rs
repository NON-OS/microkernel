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

//! The PROPERTIES op's wire, by path, laid out as the capsule mounts it.

#[path = "../../capsule_acpi/src/props/wire.rs"]
pub mod wire;
#[path = "../../capsule_acpi/src/props/wire_cursor.rs"]
mod wire_cursor;
#[path = "../../capsule_acpi/src/props/wire_read.rs"]
pub mod wire_read;
#[path = "../../capsule_acpi/src/props/wire_write.rs"]
pub mod wire_write;
