// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! What a leg of the leave phase says each round, and how it ended (rule.rs).

/// One poll of a leg that has not ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Poll {
    /// Not yet able to send (another caller holds the driver), or sent and
    /// not yet answered.
    Waiting,
    /// The driver answered with this status (0 done).
    Answered(i32),
    /// The driver is not running, or restarted since the request went.
    Gone,
}

/// How a leg ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ended {
    Answered(i32),
    Gone,
    /// Its own budget, or the total, ran out first.
    Missed,
    /// The clock could not be read, so no wait could be bounded.
    NoClock,
}
