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

//! A service that answers or asks from a worker thread is still that
//! service: the sender MkIpcRecvFrom reports is a thread, and it is
//! admitted for its process's ops, and only those.

use crate::policy::admits;

/// `power` registered from its leader 100 and runs threads 101 and 102;
/// `desktop_shell` registered from its worker 211 of process 210; 300 is
/// a thread of a process that owns nothing.
fn owner(name: &[u8]) -> Option<u32> {
    match name {
        b"power" => Some(100),
        b"desktop_shell" => Some(211),
        _ => None,
    }
}

fn group_of(pid: u32) -> Option<u32> {
    match pid {
        100..=102 => Some(100),
        210..=212 => Some(210),
        300 => Some(299),
        _ => None,
    }
}

#[test]
fn a_worker_thread_of_a_named_service_is_admitted() {
    assert!(admits(5, 102, owner, group_of), "power's second thread may prepare sleep");
    assert!(admits(3, 101, owner, group_of));
    assert!(admits(8, 210, owner, group_of), "the shell's leader, though a worker registered it");
    assert!(admits(8, 212, owner, group_of), "and its other threads");
}

#[test]
fn a_thread_admits_only_its_own_process_ops() {
    assert!(!admits(5, 212, owner, group_of), "the shell's thread may not prepare sleep");
    assert!(!admits(8, 101, owner, group_of), "power's thread may not read SMBIOS");
    assert!(!admits(3, 300, owner, group_of), "a thread of a process that owns nothing");
    assert!(!admits(9, 102, owner, group_of), "SERIAL names neither");
}
