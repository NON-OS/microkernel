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

//! Tier 2 from the suite: run the dynamically linked guest as a child and
//! read its verdict from the exit status.

use crate::exec_probe::p;
use crate::report::{Report, Seen};
use crate::sys::{call, out};

const FORK: u64 = 57;
const EXECVE: u64 = 59;
const WAIT4: u64 = 61;
const EXIT_GROUP: u64 = 231;

pub fn scan(r: &mut Report) {
    let child = call(FORK, [0; 6]);
    if child == 0 {
        let argv = [p("/bin/dyn\0"), 0u64];
        let rc = call(EXECVE, [argv[0], argv.as_ptr() as u64, 0, 0, 0, 0]);
        let _ = call(EXIT_GROUP, [(100 + (-rc).clamp(0, 100)) as u64, 0, 0, 0, 0, 0]);
    }
    let mut status = 0i32;
    let _ = call(WAIT4, [child as u64, &mut status as *mut i32 as u64, 0, 0, 0, 0]);
    let note = match (status >> 8) & 0xff {
        0 => "linked against a proved library and refused the tampered one",
        2 => {
            r.check("load a tampered library", Seen::Escaped("dlopen succeeded".into()));
            return;
        }
        3 => "ran, but the library gave a wrong answer",
        code if code >= 100 => "did not start: the loader or a library was refused",
        _ => "ended some other way",
    };
    // Only a program that ran to the dlopen saw the refusal; EPERM is what the
    // personality gives an executable mapping of unproved bytes.
    if status >> 8 & 0xff == 0 {
        r.check("load a tampered library", Seen::Refused(-1));
    }
    out(format!("[GUEST] dyn note: the dynamic program {note}\n").as_bytes());
}
