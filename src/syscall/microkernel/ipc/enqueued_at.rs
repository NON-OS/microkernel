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

//! When a message was queued, on the clock `MkTimeMonotonic` reads,
//! pure. The message is stamped on the timestamp clock; its age on that
//! clock, taken from uptime now, is its enqueue time in uptime. A stamp in
//! the future reads as now. kernel_proofs::syscall::enqueued_at.

/// `uptime_now` and `stamp_now`: the two clocks read together; `stamped`:
/// the message's timestamp.
pub(crate) const fn enqueued_uptime(uptime_now: u64, stamp_now: u64, stamped: u64) -> u64 {
    uptime_now.saturating_sub(stamp_now.saturating_sub(stamped))
}
