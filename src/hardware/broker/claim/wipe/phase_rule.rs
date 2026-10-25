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

//! Where a wipe stands and when it is due (L78). Every wait is a deadline a
//! later step checks, never a spin: the windows below are recalled from the
//! PCIe base specification (unverified) but for the two marked ours. Time
//! is the kernel's nanoseconds; a step reading a clock behind `at` waits.

pub(crate) const MS: u64 = 1_000_000;
/// How long Transactions Pending may hold the reset off (our bound).
pub(crate) const TP_BOUND: u64 = 100 * MS;
/// After an FLR, no config access; then up to a second of retry status.
pub(crate) const FLR_WAIT: u64 = 100 * MS;
pub(crate) const RETRY_WINDOW: u64 = 1000 * MS;
/// A D3hot to D0 step.
pub(crate) const PM_WAIT: u64 = 10 * MS;
/// How long a function that did not come back is refused (our bound).
pub(crate) const LOST_HOLD: u64 = 2000 * MS;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Phase {
    /// A claim stands on the function.
    Held,
    /// Its claim has gone; the reset is owed since `since`.
    Owed {
        since: u64,
    },
    /// A reset was written at `at`.
    Flr {
        at: u64,
    },
    D3 {
        at: u64,
    },
    D0 {
        at: u64,
    },
    /// Asked once more after a hold, one late answer looked for.
    Recheck,
    /// No reset can be trusted: kept for its last holder's own service.
    Kept,
    /// Not back by its deadline.
    Lost {
        at: u64,
    },
}
