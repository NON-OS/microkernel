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

//! A tampered program, carrying the proofs of the one it was made from.
//!
//! Its bytes differ from what was measured by one flipped byte, so every way
//! of running them must be refused: mapping them executable, mapping them
//! readable and then asking for exec, and exec itself. The untampered suite
//! mapped the same way is the control that the check is not simply closed.

use crate::report::{Report, Seen};
use crate::sys::{call, out, MMAP, MPROTECT, OPEN};

const PROT_READ: u64 = 1;
const PROT_EXEC: u64 = 4;
const MAP_PRIVATE: u64 = 2;

pub fn scan(r: &mut Report) {
    let fd = call(OPEN, [p("/bin/tampered\0"), 0, 0, 0, 0, 0]);
    if fd < 0 {
        r.check("open the tampered file", Seen::Refused(fd));
        return;
    }
    let map = |prot| call(MMAP, [0, 4096, prot, MAP_PRIVATE, fd as u64, 0]);
    r.check("map it executable", refused(map(PROT_READ | PROT_EXEC)));
    let at = map(PROT_READ);
    let upgraded = match at {
        a if a < 0 => a,
        a => call(MPROTECT, [a as u64, 4096, PROT_READ | PROT_EXEC, 0, 0, 0]),
    };
    r.check("map it readable, then make it executable", refused(upgraded));
    r.check("exec it", crate::exec_child::exec_in_child());
    let good = call(OPEN, [p("/bin/suite\0"), 0, 0, 0, 0, 0]);
    let proven = call(MMAP, [0, 4096, PROT_READ | PROT_EXEC, MAP_PRIVATE, good as u64, 0]);
    let note =
        if proven >= 0 { "maps executable" } else { "REFUSED: the check is closed, not proving" };
    out(format!("[GUEST] exec note: the proven suite {note}\n").as_bytes());
}

fn refused(rc: i64) -> Seen {
    match rc {
        rc if rc < 0 => Seen::Refused(rc),
        rc => Seen::Escaped(format!("succeeded at {rc:#x}")),
    }
}

pub(crate) fn p(s: &str) -> u64 {
    s.as_ptr() as u64
}
