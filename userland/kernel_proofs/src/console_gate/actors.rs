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

//! The takeovers, bootfb's hold and the whole world.

use super::world::{Drawer, Flags};

/// A takeover: its stop, its wait, its claim made ready or refused, its end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Taker {
    Off,
    Start,
    /// Between the stop's two steps, the count and the wait.
    Mid,
    /// The wait read DRAWS at 0: it believes the console quiet.
    Quiet,
    /// Its claim made ready, GONE stored, its end to come.
    Latched,
    /// Out of TAKERS with its claim still to be made ready and GONE to
    /// come: only when the gate ends a takeover before its admit.
    Freed,
    Ended {
        ready: bool,
    },
}

/// bootfb's hold: HELD stored, a wait until quiet, and HELD cleared as its
/// claim goes, which a release can do while that wait still runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Hold {
    Off,
    Start,
    Set,
    Quiet,
    /// HELD cleared by a release while the wait has yet to read DRAWS at 0.
    Waiting,
    Cleared,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct World {
    pub flags: Flags,
    pub drawers: [Drawer; 2],
    pub takers: [Taker; 2],
    pub hold: Hold,
}
