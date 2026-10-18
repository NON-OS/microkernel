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

//! How many resets in a row a device gets before the disk is given up.
//!
//! Each reset is bounded (`recover`), but the driver used to reset a device
//! on every stall for good: one that stalled every request was claimed
//! afresh for every request. Now resets count, and the count starts over
//! only once the device has served by the clock for `SERVED_SPAN_MS` since
//! the last one: any stall in that span is itself reset and starts the
//! span again, so an OK answer at its end means the device answered every
//! request it was given through it. Requests refused before the device, or
//! failed by it, do not count; nor does anything while the clock fails.
//! The stall after `RESETS_IN_A_ROW` resets ends the driver instead.

use crate::constants::{FLUSH_MS, REQUEST_MS};

/// One reset answers a lost interrupt or a host hiccup, a second a host
/// still busy after it; a third in a row with less than the span served
/// between is a device that does not serve. At most four stalls then pass
/// before the disk reads as gone, each bounded by its request's budget.
pub const RESETS_IN_A_ROW: u32 = 3;
/// Two flush budgets, a minute: longer than any one request may take, so
/// no single answer covers the span, and twelve read budgets, so a device
/// that answers once after each reset and stalls the next request is never
/// let off. One that serves the span between its stalls is reset each
/// time, as long as it does.
pub const SERVED_SPAN_MS: u64 = 2 * FLUSH_MS;
const _: () = assert!(SERVED_SPAN_MS > FLUSH_MS && SERVED_SPAN_MS >= 12 * REQUEST_MS);

#[derive(Default)]
pub struct Streak {
    resets: u32,
    /// When the last reset brought the device back, by the clock.
    since: Option<u64>,
}

impl Streak {
    /// Whether a stall may be ended by another reset.
    pub fn may_reset(&self) -> bool {
        self.resets < RESETS_IN_A_ROW
    }

    /// A reset done: the device came back at `now` (`None`: the clock
    /// failed, and then no span can be seen served from it).
    pub fn reset(&mut self, now: Option<u64>) {
        self.resets = self.resets.saturating_add(1);
        self.since = now;
    }

    /// The device answered a request OK at `now`.
    pub fn served(&mut self, now: Option<u64>) {
        let (Some(since), Some(now)) = (self.since, now) else {
            return;
        };
        if now.saturating_sub(since) >= SERVED_SPAN_MS {
            *self = Self::default();
        }
    }
}
