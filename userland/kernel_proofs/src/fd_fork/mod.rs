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
 * fork must hand the child every descriptor, close-on-exec ones included.
 *
 * The kernel's ProcessFdTable is included by path. Its fork dropped entries
 * marked close-on-exec, which only exec should close, so a child lost them
 * before it ever called exec. The check below fails against that code.
 */

#[path = "../../../../src/process/fd_types.rs"]
pub mod fd_types;
#[path = "../../../../src/process/process_fd_table.rs"]
pub mod process_fd_table;
mod tests;
