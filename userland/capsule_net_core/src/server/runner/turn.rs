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


//! What one turn of the serve loop does before it takes a request. A client
//! gives a reply 64 ms (device/budget.rs), and a turn runs to its end before
//! the next request is taken, so its driver calls must fit in that together.
//! A poll alone may spend its whole budget when the NIC driver answers late.
//! The once-a-second probes (the bound link's carrier, a better interface,
//! the Wi-Fi joins), each a driver call of its own, ran in the same turn
//! after the poll, and a client's call made then ran out. A turn now polls or
//! probes, never both: the probes take the turn they come due in, and the
//! poll runs on the next. Times are uptime, which never goes back.

/// How often the probes run.
pub const PROBE_EVERY_MS: i64 = 1000;

/// Least time between polls while the stack is quiet.
///
/// `pump` runs a full smoltcp poll: every socket, every queue, the device.
/// It used to run on every turn of this loop, so the poll rate was whatever
/// the receive wait happened to be, and a single message a second held the
/// loop attentive long enough to poll about ninety times a second forever.
/// The wait tier was never the cost; this was. Under traffic the loop is
/// attentive anyway and polls at its own rate, so this floor only applies
/// to a stack with nothing to do.
pub const IDLE_POLL_MS: i64 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Turn {
    Probe,
    Poll,
    Neither,
}

/// The turn at `now`, the probes last run at `probed` and the stack last
/// polled at `polled`; `wanted` when the stack has work (the loop is
/// attentive, or a lookup waits for its answer).
pub fn turn(now: i64, probed: i64, polled: i64, wanted: bool) -> Turn {
    if now.saturating_sub(probed) >= PROBE_EVERY_MS {
        Turn::Probe
    } else if wanted || now.saturating_sub(polled) >= IDLE_POLL_MS {
        Turn::Poll
    } else {
        Turn::Neither
    }
}
