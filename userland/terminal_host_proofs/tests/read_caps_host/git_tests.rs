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

//! The git store's read at 8 MiB: a file of it handed on whole, one byte more refused.

use nonos_git::{Storage, StorageError};

use crate::clients::vfs;
use crate::fixtures::sized;
use crate::git::storage::VfsStorage;
use crate::process::TERMINAL_PID;

const MAX: usize = 8 * 1024 * 1024;
const OBJECT: &str = ".git/objects/ab/cdef";

fn store(len: usize) -> VfsStorage {
    vfs::put(b"/home/nonos/repo/.git/objects/ab/cdef", sized(len));
    VfsStorage::new(TERMINAL_PID, "/home/nonos/repo")
}

#[test]
fn git_store_reads_a_file_of_8_mib_whole() {
    assert_eq!(store(MAX).read(OBJECT), Ok(sized(MAX)));
}

#[test]
fn git_store_refuses_a_file_one_byte_past_8_mib() {
    assert_eq!(store(MAX + 1).read(OBJECT), Err(StorageError::Io));
}

#[test]
fn git_store_says_a_missing_file_is_not_found() {
    let s = store(1);
    assert_eq!(s.root(), "/home/nonos/repo");
    assert_eq!(s.read(".git/objects/ab/none"), Err(StorageError::NotFound));
}
