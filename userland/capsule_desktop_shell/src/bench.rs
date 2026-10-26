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
//! Where the shell's start goes: each setup step as it is done, the serve
//! loop's start, and the first requests answered with how long after the
//! shell began. A caller that finds the shell's name registered at its spawn
//! waits until the serve loop runs; these lines say how long that is. No line
//! names a process.

use core::sync::atomic::{AtomicI64, AtomicU32, Ordering::Relaxed};

use nonos_libc::{mk_debug, mk_uptime_ms};

use crate::bench_line::{answered_line, mark_line};

static BEGAN_MS: AtomicI64 = AtomicI64::new(0);
static ANSWERED: AtomicU32 = AtomicU32::new(0);
/// The requests named on the serial line; those after go unmarked.
const NAMED: u32 = 4;

pub fn began() {
    BEGAN_MS.store(mk_uptime_ms(), Relaxed);
    mark(b"start");
}

pub fn mark(step: &[u8]) {
    let mut line = [0u8; 96];
    let n = mark_line(mk_uptime_ms(), step, &mut line);
    let _ = mk_debug(line.as_ptr(), n);
}

pub fn answered(op: u16) {
    if ANSWERED.fetch_add(1, Relaxed) >= NAMED {
        return;
    }
    let now = mk_uptime_ms();
    let mut line = [0u8; 96];
    let n = answered_line(op, now, now - BEGAN_MS.load(Relaxed), &mut line);
    let _ = mk_debug(line.as_ptr(), n);
}
