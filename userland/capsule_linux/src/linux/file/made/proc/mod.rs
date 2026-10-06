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
 * /proc: the family's own processes, and the system as NONOS declares it.
 *
 * A pid directory exists only for a process of the asking guest's family,
 * under the number its pid namespace gave; any other number is ENOENT, as
 * it would be for a pid that does not exist. The personality itself, the
 * namespace's pid 1, is not shown: it is not one of the family's programs.
 */

mod fds;
mod maps;
pub mod mounts;
mod names;
mod pid_files;
mod pid_status;
mod sysctl;
mod system;
mod tree;

pub use fds::{open_fds, CONSOLE_IN, CONSOLE_OUT, PIPES, SOCKETS};
pub use names::number;
pub use tree::{content, node};
