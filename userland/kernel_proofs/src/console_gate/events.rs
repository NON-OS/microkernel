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

//! Who took a step and what it was, for the trace a broken rule is
//! reported with.

use super::gate::Flag;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actor {
    Drawer(usize),
    Taker(usize),
    Hold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// A load in `stopped()`: `again` once counted, `stop` what it found.
    Load {
        flag: Flag,
        again: bool,
        stop: bool,
    },
    Count,
    BackOut,
    Write,
    Fence,
    Sub,
    /// TAKERS.fetch_add(1).
    Stop,
    /// A wait that read DRAWS at 0, or a stop that did not wait.
    Quiet,
    /// GONE.store(true), as the takeover's claim is made ready.
    Latch,
    /// TAKERS.fetch_sub(1).
    End {
        ready: bool,
    },
    /// HELD.store.
    Held(bool),
}
