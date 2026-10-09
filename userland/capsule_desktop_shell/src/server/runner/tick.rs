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

//! When the shell's housekeeping tick is due, on the uptime clock.

/// The menu bar clock shows minutes; once a second keeps it, the toasts and
/// the retries current.
pub const TICK_MS: i64 = 1000;

/// Whether a tick is due at uptime `now_ms`, the last one having run at
/// `last_ms`. A reading behind the last one (which the uptime clock should
/// never give) runs the tick rather than waiting for the clock to catch up.
pub fn tick_due(now_ms: i64, last_ms: i64) -> bool {
    now_ms < last_ms || now_ms.saturating_sub(last_ms) >= TICK_MS
}

/// The uptime by which the serve loop must run again: the next tick, or the
/// next toast's expiry when that comes first. A toast's time being up is a
/// reason to wake on its own, so it leaves the screen on time with the
/// pointer still and nothing else arriving.
pub fn wake_at(last_tick_ms: i64, toast_due_ms: Option<i64>) -> i64 {
    let tick = last_tick_ms.saturating_add(TICK_MS);
    toast_due_ms.map_or(tick, |due| due.min(tick))
}

/// How long to wait on the inbox at uptime `now_ms` for a wake at `wake_ms`:
/// never past it, never longer than `block_ms`, and at least 1 ms, since a
/// zero timeout is not a wait.
pub fn wait_ms(now_ms: i64, wake_ms: i64, block_ms: u64) -> u64 {
    let left = wake_ms.saturating_sub(now_ms).max(1) as u64;
    left.min(block_ms.max(1))
}

/// Whether the drain should go back to the loop: a stream of messages (a
/// pointer moving, an app redrawing) can arrive faster than any wait, and
/// the loop still has to run once the wake is due, so a toast whose time is
/// up leaves even then.
pub fn wake_due(now_ms: i64, wake_ms: i64) -> bool {
    now_ms >= wake_ms
}
