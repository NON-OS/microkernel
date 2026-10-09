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

//! Whether a thread's vector registers survive the switches the kernel makes
//! while it waits. Not an isolation attempt, a correctness one: two processes
//! each fill sixteen registers with their own byte and yield to each other.
//! A register that comes back holding the sibling's byte is a switch that did
//! not save it, and it is reported the way an escape is.

use crate::child_wait::wait;
use crate::report::{Report, Seen};
use crate::state_regs::{avx_usable, round_ymm};
use crate::state_regs_sse::round_xmm;
use crate::sys::call;

const FORK: u64 = 57;
const EXIT_GROUP: u64 = 231;
const ROUNDS: u32 = 300;

pub fn scan(r: &mut Report) {
    let avx = avx_usable();
    let what = if avx { "a sibling's bytes in ymm0-15" } else { "a sibling's bytes in xmm0-15" };
    let child = call(FORK, [0; 6]);
    if child == 0 {
        let lost = rounds(0xC3, avx).min(254);
        let _ = call(EXIT_GROUP, [lost as u64, 0, 0, 0, 0, 0]);
    }
    if child < 0 {
        r.check(what, Seen::Escaped(format!("fork failed, errno {}", -child)));
        return;
    }
    let mine = rounds(0x3C, avx);
    let theirs = wait(child);
    match (mine, theirs) {
        (0, Some(0)) => r.check(what, Seen::Refused(0)),
        (m, t) => r.check(what, Seen::Escaped(format!("parent {m} of {ROUNDS}, child {t:?}"))),
    }
}

// Rounds whose read-back was not the byte this process wrote.
fn rounds(byte: u8, avx: bool) -> u32 {
    let pattern = [byte; 32];
    let width = if avx { 512 } else { 256 };
    (0..ROUNDS)
        .filter(|_| {
            let mut out = [0u8; 512];
            match avx {
                // SAFETY: `avx_usable` confirmed AVX and its XCR0 state.
                true => unsafe { round_ymm(&pattern, &mut out) },
                false => round_xmm(&pattern, &mut out),
            }
            out[..width].iter().any(|b| *b != byte)
        })
        .count() as u32
}
