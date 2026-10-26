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

//! The world the model walks: a writer H that may take its error path or
//! be killed asleep in a send, a later writer C, and a reader R, around
//! PENDING (`lock`, `run`) and the volume's writer-or-readers hold (`vol`).
//! Their programs are in program.rs.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Knob {
    Fixed,
    /// The old hold: PENDING kept locked across the send.
    SendUnderLock,
    /// drop_held leaves the run as it found it.
    NoDropHeld,
    /// The volume's let_go never calls drop_held.
    NoLetGo,
}

/// A caller waits for good on a PENDING held by one asleep or dead.
pub(super) const STUCK: u8 = 1;
/// A send carries a sector its sender never held.
pub(super) const FOREIGN: u8 = 2;
/// A reader reads a sector still held.
pub(super) const PAST: u8 = 4;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) enum Ph {
    Run,
    Asleep { sending: Vec<(usize, u64)>, locked: bool, next: bool },
    Dead,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Vol {
    Free,
    Writer(usize),
    Readers(u8),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct World {
    pub(super) pc: [usize; 3],
    pub(super) ph: [Ph; 3],
    pub(super) lock: Option<usize>,
    pub(super) start: u64,
    pub(super) run: Vec<(usize, u64)>,
    pub(super) vol: Vol,
    pub(super) bad: u8,
}
