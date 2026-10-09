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

//! Whether a fetch has run out of time.

use super::budget::Budget;
use super::types::{Fetch, Phase};

/*
 * The message says what happened. A fetch that heard nothing timed out and
 * may be tried again; one that was hearing bytes and then stopped stalled,
 * and trying again only repeats the wait. Handshake bytes count: a flight
 * arriving slowly is progress, not silence.
 */
/// Why `f` has run out of time at `now`, or `None` while it has not.
pub fn due(f: &Fetch, b: &Budget, now: i64) -> Option<&'static str> {
    let quiet = now.wrapping_sub(f.progress_ms);
    if f.phase == Phase::Connecting {
        return (quiet > b.connect_ms).then_some("connect failed");
    }
    if f.keep_uses > 0 && f.received == 0 && quiet > b.reuse_ms {
        return Some("kept connection dead");
    }
    if quiet <= b.silent_ms && now.wrapping_sub(f.started_ms) <= b.total_ms {
        return None;
    }
    Some(if f.received == 0 { "timed out" } else { "stalled" })
}

/// How long a read may take now: the read budget, cut short by `until`.
pub fn read_ms(now: i64, until: i64) -> i64 {
    until.wrapping_sub(now).clamp(1, super::constants::DRAIN_MS)
}
