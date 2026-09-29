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
 * A procfs pid directory must not take its inode from a negative pid.
 *
 * The kernel's pid_dir_inode is included by path. lookup_root used to compute
 * pid as u64 * 1000 + 100 on any parsed i32, so the name -1 overflowed. The
 * check below does not build against that code, which had no such function.
 */

#[path = "../../../../src/fs/procfs/pid_inode.rs"]
pub mod pid_inode;
mod tests;
