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

//! Telling newcomers BUSY while a session runs, and letting each one go when
//! turn_away.rs says it is done. Every call here is one net.sockets round
//! trip and none waits, except `let_go`, which runs between sessions.

use crate::read::Read;
use crate::sock::{Accept, Net, Recv};
use crate::turn_away::{Away, LET_GO_IDLE_MS, LET_GO_PASSES};
use nonos_libc::{mk_idle_ms, mk_uptime_ms};
use nsh_server::busy;

/// Monotonic milliseconds since boot, the clock every hold is timed on.
pub fn uptime() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}

/// One round, when one is due: take a newcomer from `listener`, if one is
/// waiting, and tell it BUSY; then read each one held and close those done.
pub fn round(net: &mut Net, listener: u32, away: &mut Away) {
    let now = uptime();
    if !away.due(now) {
        return;
    }
    match net.accept(listener) {
        Accept::Child(child) => tell(net, away, child, now, &busy()),
        Accept::Pending => {}
        Accept::Gone => away.lose_listener(),
    }
    settle(net, away, now);
}

/// Tell `child` the plaintext `answer` (BUSY, or why the host serves no one)
/// and hold it; close it at once when the answer cannot be sent or no slot
/// is free.
pub fn tell(net: &mut Net, away: &mut Away, child: u32, now: u64, answer: &[u8]) {
    if !net.send(child, answer) {
        return net.close(child);
    }
    if let Some(full) = away.hold(child, now) {
        net.close(full);
    }
}

/// Read every newcomer held once, and close each one that is done.
fn settle(net: &mut Net, away: &mut Away, now: u64) {
    let seq = away.seq();
    for handle in away.held().into_iter().flatten() {
        let read = match net.recv(handle, seq) {
            Recv::Data(_) => Read::Data,
            Recv::Idle => Read::Idle,
            Recv::Gone => Read::Gone,
        };
        if away.settle(handle, read, now) {
            net.close(handle);
        }
    }
}

/// Wait until no newcomer is held: each goes when its peer does, or after
/// turn_away's HOLD_MS at most. After LET_GO_PASSES reads whatever is left
/// is closed, whatever the clock said.
pub fn let_go(net: &mut Net, away: &mut Away) {
    let mut passes = 0u64;
    while away.holding() && passes < LET_GO_PASSES {
        away.next_round();
        settle(net, away, uptime());
        passes += 1;
        if away.holding() {
            let _ = mk_idle_ms(LET_GO_IDLE_MS);
        }
    }
    for handle in away.release_all().into_iter().flatten() {
        net.close(handle);
    }
}
