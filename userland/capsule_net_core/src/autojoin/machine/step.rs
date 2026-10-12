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

//! What a pass came to, what is due, and where the machine is.

/// How one pass went.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Not yet possible: no policy, no driver, no store, or a driver not ready.
    NotYet,
    /// Ready, but no due saved network was heard.
    NoneInRange,
    /// Saved network `index` was handed to the driver and no answer came
    /// within `CALL_MS`; the join runs on in the driver.
    Started(u8),
    /// Saved network `index` was tried and the driver answered at once.
    Answered { index: u8, joined: bool },
    /// Nothing left to do this boot.
    Done,
}

/// What is due at a given time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Due {
    Nothing,
    /// Run one pass (`Step`).
    Attempt,
    /// Look at the link of the join in progress.
    Watch,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Phase {
    Waiting,
    Joining { index: u8, until_ms: i64 },
    Done,
}
