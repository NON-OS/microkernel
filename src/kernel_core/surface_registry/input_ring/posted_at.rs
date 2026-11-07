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

//! The time an event was posted, which the kernel writes into its
//! `timestamp_ns` over whatever the poster put there, so an app that times
//! two presses (a double click on a title bar, a word picked in the Terminal)
//! measures when they came, not when it got round to reading them, and no
//! poster can date an event. Nanoseconds since boot on the clock
//! `mk_uptime_ms` reads, at its millisecond grain, so an app can compare an
//! event's time with its own clock. Proven in kernel_proofs::input_posted_at.

/// `since_boot_ms` as the nanoseconds put on an event; the last
/// representable nanosecond for a time past it, never a wrapped one.
pub(crate) fn posted_ns(since_boot_ms: u64) -> u64 {
    since_boot_ms.saturating_mul(1_000_000)
}
