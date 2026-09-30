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

/*
 * Saying the ring `recent` keeps: after a guest thread's death on a signal,
 * and after a process that exited with a nonzero status.
 */

use alloc::string::String;
use core::fmt::Write;
use core::sync::atomic::Ordering::Relaxed;

use super::recent::{CALLS, KEEP, NEXT};
use crate::linux::abi::{nr, nr_high};

fn lays_out_memory(number: u64) -> bool {
    matches!(number, nr::MMAP | nr::MPROTECT | nr::MUNMAP | nr::BRK | nr_high::MREMAP)
}

/// `[LINUX] last calls: 9(0)=0x100000000 12 ...`, oldest first.
pub fn say() {
    let held = crate::linux::console::private();
    let mut line = String::from("[LINUX] last calls:");
    for i in 0..KEEP {
        let at = (NEXT.load(Relaxed) + i) % KEEP * 3;
        let [number, arg0, result] = [0, 1, 2].map(|k| CALLS[at + k].load(Relaxed));
        if number == u64::MAX {
            continue;
        }
        let _ = match !held || lays_out_memory(number) {
            true => write!(line, " {number}({arg0:#x})={result:#x}"),
            false => write!(line, " {number}"),
        };
    }
    line.push('\n');
    crate::linux::start::say(line.as_bytes());
}

/// A guest thread ended on a signal: that, then the calls before it.
pub fn died(pid: u32) {
    let line = alloc::format!("[LINUX] guest thread {pid} ended on a signal; ending the process\n");
    crate::linux::start::say(line.as_bytes());
    say();
}
