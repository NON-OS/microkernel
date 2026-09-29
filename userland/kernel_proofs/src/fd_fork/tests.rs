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

use super::fd_types::FdEntry;
use super::process_fd_table::ProcessFdTable;

#[test]
fn fork_keeps_close_on_exec_descriptors_until_exec() {
    let parent = ProcessFdTable::new();
    let keep = parent.allocate(FdEntry::with_pipe(1, true)).expect("fd");
    let cloexec = parent.allocate(FdEntry::with_pipe(1, false)).expect("fd");
    assert!(parent.set_cloexec(cloexec, true));

    let child = parent.fork();
    assert!(child.is_valid(keep));
    assert!(child.is_valid(cloexec));
    assert_eq!(child.get_cloexec(cloexec), Some(true));

    child.close_cloexec();
    assert!(child.is_valid(keep));
    assert!(!child.is_valid(cloexec));
    assert!(parent.is_valid(cloexec));
}
