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

//! What a receive does after finding its inbox empty, pure. A
//! `timeout_ms` of 0 waits for ever, as it always has; NO_WAIT looks once;
//! anything else waits that long. kernel_proofs::syscall::microkernel::wait.

/// The timeout that looks once and returns EAGAIN on an empty inbox.
pub(crate) const NO_WAIT: u64 = u64::MAX;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Next {
    /// Empty, and the caller asked only to look: EAGAIN.
    Again,
    /// The wait ran out: ETIMEDOUT.
    TimedOut,
    /// Sleep until this timestamp, or a delivery.
    SleepUntil(u64),
}

/// After an empty look `elapsed` ms past `start`.
pub(crate) const fn after_empty(timeout_ms: u64, start: u64, elapsed: u64) -> Next {
    if timeout_ms == NO_WAIT {
        return Next::Again;
    }
    if timeout_ms > 0 && elapsed >= timeout_ms {
        return Next::TimedOut;
    }
    if timeout_ms == 0 {
        Next::SleepUntil(u64::MAX)
    } else {
        Next::SleepUntil(start.saturating_add(timeout_ms))
    }
}
