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

/* Every inode under a pid directory is (pid << PID_INODE_SHIFT) | k: the
directory itself is k = 0 and its entries k = 1 to 103. Root entries sit below
1 << PID_INODE_SHIFT. Only pids from 1 have a directory, so none of these
numbers is the root's, a root entry's or another pid's. */
pub(crate) const PID_INODE_SHIFT: u32 = 20;

pub(crate) fn pid_dir_inode(pid: i32) -> Option<u64> {
    if pid <= 0 {
        return None;
    }
    Some((pid as u64) << PID_INODE_SHIFT)
}
