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

//! The record /proc keeps of what each process runs lives as long as the
//! process. A family running command after command, each a fork and an
//! exec that then ends, holds a record for the processes alive and no
//! more, and a record that is gone is never answered.

use crate::linux::file::exe::shape::Exe;
use crate::linux::file::exe::table::{forget, of, record};

fn exe(name: &[u8]) -> Exe {
    Exe { path: name.to_vec(), comm: name.to_vec(), ..Exe::default() }
}

/// One test alone uses the table, which is one static as in the capsule.
#[test]
fn a_record_lives_as_long_as_its_process() {
    record(2, exe(b"/bin/sh"));
    for pid in 3..100_003u32 {
        record(pid, exe(b"/bin/ls"));
        assert_eq!(of(pid).map(|e| e.path), Some(b"/bin/ls".to_vec()));
        forget(pid);
        assert!(of(pid).is_none(), "an ended process is never answered for");
    }
    assert_eq!(of(2).map(|e| e.path), Some(b"/bin/sh".to_vec()), "the shell runs on");
    record(2, exe(b"/bin/busybox"));
    assert_eq!(of(2).map(|e| e.comm), Some(b"/bin/busybox".to_vec()), "an exec replaces");
    forget(2);
    assert!(of(2).is_none());
}
