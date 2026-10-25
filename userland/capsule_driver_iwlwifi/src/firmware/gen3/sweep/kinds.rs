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

//! A sweep's budget, why one could not start, and what a call left it at.

use super::super::dev::WaitError;

/// Per-channel allowance: the 110 ms passive dwell and the switch.
const PER_CHANNEL_MS: u32 = 150;
const MARGIN_MS: u32 = 2000;

/// How long a sweep over `channels` channels may run before it is given up.
pub fn budget_ms(channels: usize) -> u32 {
    let n = u32::try_from(channels).unwrap_or(u32::MAX);
    PER_CHANNEL_MS.saturating_mul(n).saturating_add(MARGIN_MS)
}

/// Why a sweep could not start.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScanError {
    /// No channels, or more than one request carries.
    NoChannels,
    /// The scan request was not answered (or the firmware failed meanwhile).
    Request(WaitError),
}

/// What one call left the sweep at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tick {
    /// No sweep is in flight (the buffers were still drained).
    Idle,
    /// The sweep is still running.
    Running,
    /// The firmware reported this scan complete with this status.
    Completed(u8),
    /// The sweep ran past its budget with no completion.
    Stalled,
    /// The firmware or hardware raised its error cause.
    Failed,
}
