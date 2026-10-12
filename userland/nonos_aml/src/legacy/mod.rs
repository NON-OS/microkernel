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

//! Legacy PC devices as the namespace declares them (ACPI 6.5, 5.2.9.3 and
//! 6.4), for the drivers that would otherwise assume their fixed ports.

mod boot_arch;
mod find;
mod io;
mod kind;

pub use boot_arch::{boot_arch, BootArch};
pub use find::{has_8042, legacy_devices, Legacy};
pub use io::{io_ranges, memory_ranges, Range};
pub use kind::{kind_of, LegacyKind};
