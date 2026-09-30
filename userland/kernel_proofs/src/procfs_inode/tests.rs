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
fn only_pids_from_one_have_a_directory_inode() {
    assert_eq!(pid_dir_inode(-1), None);
    assert_eq!(pid_dir_inode(i32::MIN), None);
    assert_eq!(pid_dir_inode(0), None);
    assert_eq!(pid_dir_inode(1), Some(1 << 20));
    assert_eq!(pid_dir_inode(i32::MAX), Some((i32::MAX as u64) << 20));
}

#[test]
fn directory_inodes_miss_root_and_entry_inodes() {
    for pid in 1..4096 {
        let ino = pid_dir_inode(pid).expect("inode");
        assert!(ino >= 1 << 20, "pid {pid} reaches the root entries");
        assert_eq!(ino & 0xF_FFFF, 0, "pid {pid} shares an entry's low bits");
    }
    assert_ne!(pid_dir_inode(131072), Some((125 << 20) | 100));
}
