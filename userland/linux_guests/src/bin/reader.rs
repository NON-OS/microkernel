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

//! The half of the memory pair that goes looking.
//!
//! Every way Linux offers one process into another's memory, tried against
//! every pid a sibling could plausibly have, then the blunt one: map the same
//! address and see whose page arrives.

use std::process::ExitCode;

use nonos_linux_guests::report::{Report, Seen};
use nonos_linux_guests::sys::{
    call, GETPID, MAP_FIXED, MAP_PRIVATE_ANON, MMAP, PATTERN, PATTERN_AT, PROT_RW,
};
use nonos_linux_guests::{proc_probe, vm_probe};

/// Guest pids are small; a sibling started beside this one sits well inside.
const PID_RANGE: u32 = 256;

fn main() -> ExitCode {
    let mut r = Report::new("reader");
    let me = call(GETPID, [0; 6]) as u32;
    let pids: Vec<u32> = (1..=PID_RANGE).filter(|p| *p != me).collect();
    vm_probe::scan(&mut r, &pids);
    proc_probe::scan(&mut r, &pids);
    let flags = MAP_PRIVATE_ANON | MAP_FIXED;
    let at = call(MMAP, [PATTERN_AT, 4096, PROT_RW, flags, u64::MAX, 0]);
    let seen = if at != PATTERN_AT as i64 {
        Seen::Refused(at.min(-1))
    } else {
        // SAFETY: mapped read-write just above.
        let page = unsafe { core::slice::from_raw_parts(PATTERN_AT as *const u8, 16) };
        match page == PATTERN {
            true => Seen::Escaped("the sibling's page came back".into()),
            false => Seen::Refused(0),
        }
    };
    r.check("same address, own page", seen);
    r.finish()
}
