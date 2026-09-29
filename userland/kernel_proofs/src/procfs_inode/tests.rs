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

use super::pid_inode::pid_dir_inode;

#[test]
fn negative_pids_have_no_directory_inode() {
    assert_eq!(pid_dir_inode(-1), None);
    assert_eq!(pid_dir_inode(i32::MIN), None);
    assert_eq!(pid_dir_inode(0), Some(100));
    assert_eq!(pid_dir_inode(i32::MAX), Some(2_147_483_647_100));
    for pid in 0..2048 {
        assert_eq!(pid_dir_inode(pid), Some(pid as u64 * 1000 + 100));
    }
}
