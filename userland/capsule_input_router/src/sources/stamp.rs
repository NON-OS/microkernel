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

//! An event's time, where the kernel ring hands it over. The input ABI
//! carries a time in nanoseconds (abi/wire.toml [input.event_v1]), which
//! the kernel writes as each event is posted (input_ring/posted_at.rs).
//! An event that still comes with none takes the time it was drained, so
//! no event reaches its window as time 0, which the Terminal and the Text
//! Editor, timing a double click by it, would take as one with any press
//! before it on the same spot. A time the event has is kept.

use nonos_libc::InputEvent;

const NS_PER_MS: u64 = 1_000_000;

/// Each of `events` with no time of its own takes `now_ms`, the uptime the
/// batch was drained at, in nanoseconds. A clock read below zero or too
/// large for nanoseconds leaves the time unknown (0), as before.
pub fn stamp(events: &mut [InputEvent], now_ms: i64) {
    let Some(ns) = u64::try_from(now_ms).ok().and_then(|ms| ms.checked_mul(NS_PER_MS)) else {
        return;
    };
    for ev in events.iter_mut().filter(|e| e.timestamp_ns == 0) {
        ev.timestamp_ns = ns;
    }
}
