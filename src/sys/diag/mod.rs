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

//! Trust-boundary diagnostics.
//!
//! A refusal at a capability gate is invisible from outside: the caller sees a
//! timeout and retries, and the system reads as slow rather than as denying.
//! Every gate that can refuse therefore reports here, with who asked, what
//! they held, and what the gate wanted.
//!
//! Healthy boots emit nothing from these sites, so the lines are signal, not
//! noise. Each site is rate limited on its own counter: the first `VERBOSE`
//! occurrences print, after that every `SAMPLE`th, so a capsule spinning on a
//! refused call cannot flood the console it is being diagnosed from.

use core::sync::atomic::{AtomicU32, Ordering};

use super::serial::Line;

mod late;
mod reply;

/// One instrumented gate. Declare as a `static` at the call site so each gate
/// rate-limits independently.
pub struct Site {
    label: &'static [u8],
    hits: AtomicU32,
}

const VERBOSE: u32 = 64;
const SAMPLE: u32 = 512;

impl Site {
    pub const fn new(label: &'static [u8]) -> Site {
        Site { label, hits: AtomicU32::new(0) }
    }

    fn admit(&self) -> Option<u32> {
        let n = self.hits.fetch_add(1, Ordering::Relaxed);
        (n < VERBOSE || n % SAMPLE == 0).then_some(n)
    }

    /// "[DIAG] <label>", the start of each of this site's lines. A line is
    /// built whole and written once (serial/line.rs), so no other CPU's line
    /// lands inside it.
    fn head(&self) -> Line {
        let mut l = Line::new();
        l.str(b"[DIAG] ").str(self.label);
        l
    }

    /// A capability gate refused `pid`: it held `have`, the gate wanted `need`.
    pub fn refused(&self, what: &str, pid: u32, have: u64, need: u64) {
        let Some(n) = self.admit() else { return };
        let mut l = self.head();
        l.str(b" refused ").str(what.as_bytes()).str(b" pid=").hex(pid as u64);
        trail(l.str(b" have=").hex(have).str(b" need=").hex(need), n);
    }

    /// The gate rejected `what` for a reason other than capabilities.
    pub fn reject(&self, what: &str, why: &'static str, pid: u32) {
        let Some(n) = self.admit() else { return };
        let mut l = self.head();
        l.str(b" ").str(why.as_bytes()).str(b" ").str(what.as_bytes());
        trail(l.str(b" pid=").hex(pid as u64), n);
    }

    /// A call that was sent and never answered: `errno` from the receive.
    /// Endpoint name and errno only: no pid, as for every kernel line that
    /// can leave the machine in a photo or a bug report.
    pub fn starved(&self, what: &str, errno: i64) {
        let Some(n) = self.admit() else { return };
        let mut l = self.head();
        l.str(b" unanswered ").str(what.as_bytes()).str(b" err=");
        if errno < 0 {
            l.str(b"-");
        }
        trail(l.dec(errno.unsigned_abs()), n);
    }

    /// A state transition worth one line, `what` = `value`.
    pub fn note(&self, what: &str, value: u64) {
        let Some(n) = self.admit() else { return };
        let mut l = self.head();
        trail(l.str(b" ").str(what.as_bytes()).str(b"=").hex(value), n);
    }
}

/// The sample count once past `VERBOSE`, then the line, in one write.
fn trail(l: &mut Line, n: u32) {
    if n >= VERBOSE {
        l.str(b" hits=").hex(n as u64);
    }
    l.end();
}
