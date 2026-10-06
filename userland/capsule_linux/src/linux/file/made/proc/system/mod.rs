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
 * /proc's system-wide files, as NONOS declares the system to a family.
 *
 * The machine is one CPU and the family's memory limit, up since the
 * family started. What the family used of it is the kernel's measure of
 * the family's own threads; nothing of any other process, capsule or
 * family, and nothing of the hardware, is in any of these files.
 */

mod files;
mod memory;
mod stat;
mod time;

pub use files::content;
