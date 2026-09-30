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
 * A procfs pid directory's inode must be no other inode.
 *
 * The kernel's pid_dir_inode is included by path. The directory inode was
 * pid * 1000 + 100: the name -1 overflowed it, pid 0 got the root sys entry's
 * inode, and pid 131072 got pid 125's task entry inode. The checks below fail
 * against that numbering.
 */

#[path = "../../../../src/fs/procfs/pid_inode.rs"]
pub mod pid_inode;
mod tests;
