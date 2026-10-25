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

//! A second thread that looks at the serve loop's breadcrumbs every
//! WATCH_MS and says, once, a turn stuck in one step: a stuck turn never
//! ends, so the loop's own slow-turn line never comes. Its lines are built
//! on its stack, not the heap, whose lock the stuck loop may hold. Silence
//! from it while callers go unanswered means the process is not running.

use nonos_libc::{mk_debug, mk_idle_ms, mk_uptime_ms, WorkerSeat};

use super::breadcrumb::read;
use super::stall_line::line;
use super::stall_rule::StallRule;

const WATCH_MS: u64 = 500;

static SEAT: WorkerSeat = WorkerSeat::new();

/// Start the watchdog; without a thread the loop runs unwatched.
pub fn start() {
    if SEAT.spawn(watch, 0).is_err() {
        let line = b"[DISPLAY] stall watch: no thread, the loop runs unwatched\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}

fn watch(_: usize) {
    let mut rule = StallRule::default();
    loop {
        let _ = mk_idle_ms(WATCH_MS);
        if let Some(say) = rule.check(read(), mk_uptime_ms()) {
            let (buf, len) = line(say);
            let _ = mk_debug(buf.as_ptr(), len);
        }
    }
}
