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

//! A Linux guest reaching for the NØNOS native ABI.
//!
//! The guest holds no capabilities, and its syscalls are meant to reach the
//! personality and nothing else. So each native call here, with arguments
//! that would work for a capsule, must fail. One that returns success means
//! a Linux program can talk to ring 0 as a capsule does.

use crate::arg::{p, pm, pu};
use crate::report::{Report, Seen};
use crate::sys::call;

const fn tag4(b: &[u8; 4]) -> u64 {
    (b[0] as u64) | ((b[1] as u64) << 8) | ((b[2] as u64) << 16) | ((b[3] as u64) << 24)
}

pub fn scan(r: &mut Report) {
    let mut key = [0u8; 32];
    let label = b"guest/probe";
    let name = b"vfs_pool";
    let (mut port, mut pid) = (0u32, 0u32);
    let msg = [0u8; 16];
    let token = [0u8; 32];
    let mut entries = [0u8; 256];
    let probes: [(&str, u64, [u64; 6]); 6] = [
        ("machine key", tag4(b"CMKY"), [p(label), label.len() as u64, pm(&mut key), 0, 0, 0]),
        (
            "service lookup",
            tag4(b"MSVL"),
            [p(name), name.len() as u64, pu(&mut port), pu(&mut pid), 0, 0],
        ),
        ("ipc send", tag4(b"MISD"), [1, p(&msg), msg.len() as u64, 0, 0, 0]),
        (
            "debug console",
            tag4(b"MDBG"),
            [p(b"[GUEST] native wrote the console\n"), 34, 0, 0, 0, 0],
        ),
        ("consent restore", tag4(b"MLCR"), [p(&token), 0, 0, 0, 0, 0]),
        ("attest entries", tag4(b"MAEN"), [pm(&mut entries), entries.len() as u64, 0, 0, 0, 0]),
    ];
    for (what, nr, args) in probes {
        let rc = call(nr, args);
        let seen = match rc {
            rc if rc < 0 => Seen::Refused(rc),
            rc => Seen::Escaped(format!("returned {rc}")),
        };
        r.check(what, seen);
    }
    if key != [0u8; 32] {
        r.check("machine key bytes", Seen::Escaped("the buffer was filled".into()));
    }
}
