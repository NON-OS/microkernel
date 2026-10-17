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

//! D13b: the kernel names the disk its block layer chose, and the installer
//! never offers it. The kernel's record and nonos_blk_client's reading of it
//! are included by path, so the two are held to one layout.

#[path = "../../../../../src/syscall/microkernel/install_source_in_use.rs"]
mod kernel;
#[path = "../../../../nonos_blk_client/src/disks/in_use.rs"]
mod installer;

mod tests;
