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

//! How long a drain waits for the next request after serving one, pure.
//!
//! A timed receive that finds its inbox empty sleeps until a CPU's timer
//! sweep wakes it, and that sweep runs at the scheduler's tick (100 Hz on
//! x86_64), so even a 1 ms wait can hold a served request up to 10 ms
//! before its frame is composed. A delivery wakes the receive at once.
//!
//! So once the last frame is at least FRAME_FLOOR_MS old the drain only
//! looks: an event that arrives alone is composed straight away. Inside the
//! floor it waits out the rest of it, and every request that lands in that
//! time wakes it and is served in the same frame, so input at any rate is
//! composed at most once a floor, never once an event.

/// The least time from one frame to the next that input alone brings on:
/// two frames for each refresh of a 60 Hz display.
pub const FRAME_FLOOR_MS: i64 = 8;

/// When the last passes ran (uptime ms): one that served a request or drew,
/// and one that drew.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Last {
    pub served: i64,
    pub drew: i64,
}

/// The pass a follow-up wait counts the floor from. A drain holding damage
/// counts it from the last pass that drew: a pass that served only requests
/// that draw nothing (a pointer moved on the cursor plane, a query) has no
/// frame to space this one from, and a title-bar drag's scene submit, just
/// behind its pointer move, waited out that pass's floor, and the timer
/// sweep after it, before its frame was composed. A drain with no damage yet
/// counts from the last pass of either kind, so pointer moves alone still go
/// to the plane at most once a floor.
pub const fn paced_from(last: Last, damaged: bool) -> i64 {
    if damaged {
        last.drew
    } else {
        last.served
    }
}

/// None looks once; Some(ms) waits up to that long. `composed_at` is when the
/// last pass composed. A clock read that went back waits the whole floor.
pub const fn follow_up(now: i64, composed_at: i64) -> Option<u64> {
    let since = now.saturating_sub(composed_at);
    if since >= FRAME_FLOOR_MS {
        None
    } else if since < 0 {
        Some(FRAME_FLOOR_MS as u64)
    } else {
        Some((FRAME_FLOOR_MS - since) as u64)
    }
}
