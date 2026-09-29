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
 * The last calls a family made, said when a guest dies on a signal, so a
 * crash names what the program was doing rather than only that it ended.
 * Only numbers are kept, never bytes of the guest's memory. A family that
 * holds a model shows results only for the calls that lay out memory,
 * which are addresses and lengths, not anything the model was told.
 */

use alloc::string::String;
use core::fmt::Write;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed};

use crate::linux::abi::{nr, nr_high};

const KEEP: usize = 16;

/* Number, first argument and result of each call, in a ring. */
static CALLS: [AtomicU64; KEEP * 3] = [const { AtomicU64::new(u64::MAX) }; KEEP * 3];
static NEXT: AtomicUsize = AtomicUsize::new(0);

/// Note one answered call: its number, first argument and result.
pub fn note(number: u64, arg0: u64, result: u64) {
    let at = NEXT.load(Relaxed);
    for (i, v) in [number, arg0, result].into_iter().enumerate() {
        CALLS[at * 3 + i].store(v, Relaxed);
    }
    NEXT.store((at + 1) % KEEP, Relaxed);
}

fn lays_out_memory(number: u64) -> bool {
    matches!(number, nr::MMAP | nr::MPROTECT | nr::MUNMAP | nr::BRK | nr_high::MREMAP)
}

/// `[LINUX] last calls: 9(0)=0x100000000 12 ...`, oldest first.
pub fn say() {
    let held = crate::linux::file::models::held();
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
