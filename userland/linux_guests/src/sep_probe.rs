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

//! Two address spaces after a fork, and nothing passing between them.
//!
//! A child starts with a copy of its parent's memory; that is fork. What must
//! not happen after is a write on one side appearing on the other, or either
//! reaching into the other. The child reports through its exit status.

use crate::report::{Report, Seen};
use crate::sys::{call, GETPID, MAP_FIXED, MAP_PRIVATE_ANON, MMAP, PROT_RW};

const FORK: u64 = 57;
const EXIT_GROUP: u64 = 231;
const AT: u64 = 0x5000_0000;

pub fn scan(r: &mut Report) {
    let fixed = MAP_PRIVATE_ANON | MAP_FIXED;
    if call(MMAP, [AT, 4096, PROT_RW, fixed, u64::MAX, 0]) != AT as i64 {
        r.check("map the shared-looking page", Seen::Refused(-12));
        return;
    }
    poke(b'A');
    let parent = call(GETPID, [0; 6]) as u32;
    let child = call(FORK, [0; 6]);
    if child == 0 {
        let _ = call(EXIT_GROUP, [super::sep_child::run(parent), 0, 0, 0, 0, 0]);
    }
    if child < 0 {
        r.check("fork", Seen::Refused(child));
        return;
    }
    poke(b'B');
    let mut status = 0i32;
    let Some(bits) = super::sep_child::heard(r, child, &mut status) else {
        return;
    };
    let seen = |bit: i32, how: &str| match bits & bit {
        0 => Seen::Refused(0),
        _ => Seen::Escaped(how.into()),
    };
    r.check("parent write reaching the child", seen(1, "the child saw B"));
    r.check("child reading the parent", seen(2, "process_vm_readv or ptrace worked"));
    let back = peek();
    r.check(
        "child write reaching the parent",
        match back {
            b'C' => Seen::Escaped("the parent saw C".into()),
            _ => Seen::Refused(0),
        },
    );
}

pub(crate) fn poke(v: u8) {
    // SAFETY: AT was mapped read-write for 4096 bytes before any call here.
    unsafe { core::ptr::write_volatile(AT as *mut u8, v) }
}

pub(crate) fn peek() -> u8 {
    // SAFETY: as for poke.
    unsafe { core::ptr::read_volatile(AT as *const u8) }
}
