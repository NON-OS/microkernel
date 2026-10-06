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

//! Where a sent signal goes: queued here for the caller's own process, or
//! through the outbox with the caller parked, for the family to route and
//! answer once it knows whether anyone was there.

use crate::linux::abi::errno;
use crate::linux::guest::siginfo::SigInfo;
use crate::linux::guest::sigwaits::{Outbound, Target};
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// Queue `info` for `to`: here when it is this process, else through the
/// family. A signal number of 0 only asks whether the target exists.
pub fn post(guest: &mut Guest, tid: u32, to: Target, info: SigInfo) -> Answer {
    let here = match to {
        Target::Process(p) if guest.owns(p) => Some(0),
        Target::Thread(g, t) if guest.owns(t) && (g == 0 || g == guest.pid) => Some(t),
        _ => None,
    };
    let Some(t) = here else {
        guest.signals.outbox.push(Outbound { from: tid, to, info });
        return Answer::Park;
    };
    /* A realtime signal past the queue limit is refused, as Linux does. */
    let s = info.signo;
    if s != 0 && !guest.raise_and_wake(t, info) && s >= 32 && !guest.signals.discards(s) {
        return Answer::value(errno::fail(errno::EAGAIN));
    }
    Answer::value(errno::ok(0))
}
