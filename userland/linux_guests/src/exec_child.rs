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

//! The exec half of the tamper probe, in a child of its own.

use crate::exec_probe::p;
use crate::report::Seen;
use crate::sys::call;

const FORK: u64 = 57;
const EXECVE: u64 = 59;
const WAIT4: u64 = 61;
const EXIT_GROUP: u64 = 231;
const NOT_REFUSED: i32 = 99;

/// exec replaces the process that calls it, so a child makes the attempt and
/// reports an errno through its status; a status under 100 is a program
/// that ran.
pub fn exec_in_child() -> Seen {
    let child = call(FORK, [0; 6]);
    if child == 0 {
        let argv = [p("/bin/tampered\0"), 0u64];
        let rc = call(EXECVE, [argv[0], argv.as_ptr() as u64, 0, 0, 0, 0]);
        // Only a negative errno is a refusal; anything else returned is not.
        let code = if rc < 0 { 100 + (-rc).min(100) } else { NOT_REFUSED as i64 };
        let _ = call(EXIT_GROUP, [code as u64, 0, 0, 0, 0, 0]);
    }
    let mut status = 0i32;
    let _ = call(WAIT4, [child as u64, &mut status as *mut i32 as u64, 0, 0, 0, 0]);
    match (status >> 8) & 0xff {
        code if code >= 100 => Seen::Refused(-(code as i64 - 100)),
        NOT_REFUSED => Seen::Escaped("execve returned without refusing".into()),
        code => Seen::Escaped(format!("the tampered program ran and exited {code}")),
    }
}
