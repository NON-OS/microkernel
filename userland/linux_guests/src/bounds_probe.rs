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

//! A guest asking for more than any plan should give it.
//!
//! Each request is one a bounded address-space plan refuses: honouring it
//! is the failure, not just crashing on it. A refusal must come back as an
//! errno to this process, with the machine still up to print the next line.

use crate::report::{Report, Seen};
use crate::sys::{call, BRK, MAP_FIXED, MAP_PRIVATE_ANON, MMAP, MPROTECT, PROT_RW};

const TIB: u64 = 1 << 40;
/// The first address of the kernel half on x86_64.
const KERNEL_HALF: u64 = 0xffff_8000_0000_0000;
/// No sane plan maps page zero; a guest that gets it can make null pointers
/// point somewhere.
const PAGE_ZERO: u64 = 0;

pub fn scan(r: &mut Report) {
    let rc = call(MMAP, [0, TIB, PROT_RW, MAP_PRIVATE_ANON, u64::MAX, 0]);
    r.check("mmap one TiB", granted(rc, "mapped a TiB"));
    let fixed = MAP_PRIVATE_ANON | MAP_FIXED;
    let rc = call(MMAP, [KERNEL_HALF, 4096, PROT_RW, fixed, u64::MAX, 0]);
    r.check("mmap in the kernel half", granted(rc, "mapped the kernel half"));
    let rc = call(MMAP, [PAGE_ZERO, 4096, PROT_RW, fixed, u64::MAX, 0]);
    r.check("mmap page zero", granted(rc, "mapped page zero"));
    let base = call(BRK, [0; 6]);
    let rc = call(BRK, [(base as u64).wrapping_add(TIB), 0, 0, 0, 0, 0]);
    // brk reports failure by returning the old break, not an errno.
    let seen = match rc > base && base > 0 {
        true => Seen::Escaped(format!("break moved by {:#x}", rc - base)),
        false => Seen::Refused(-12),
    };
    r.check("brk one TiB", seen);
    let rc = call(MPROTECT, [KERNEL_HALF, 4096, PROT_RW, 0, 0, 0]);
    r.check("mprotect the kernel half", granted(rc, "changed kernel protections"));
}

/// A mapping call succeeded when it returned an address, not an errno.
fn granted(rc: i64, what: &str) -> Seen {
    match rc {
        rc if (-4095..0).contains(&rc) => Seen::Refused(rc),
        rc => Seen::Escaped(format!("{what} at {rc:#x}")),
    }
}
