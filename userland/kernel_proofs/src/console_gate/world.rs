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

//! The four atomics, each read and written whole, and where a drawer is.
//! Every rule is a function of a state or of a step taken from one, so a
//! state reached twice is searched once.

use super::gate::Flag;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Flags {
    pub held: bool,
    pub gone: bool,
    pub takers: u8,
    pub draws: u8,
}

impl Flags {
    /// What one load of `flag` finds: true when it says stop.
    pub fn stop(&self, flag: Flag) -> bool {
        match flag {
            Flag::Held => self.held,
            Flag::Gone => self.gone,
            Flag::Takers => self.takers > 0,
        }
    }
}

/// Where a drawer is in `drawable()`, its draw and its guard's drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum At {
    /// Before load `n` of the check made before the count.
    First(u8),
    Count,
    /// Before load `n` of the check made once counted.
    Again(u8),
    BackOut,
    Write,
    /// Before step `n` of the drop's two, the fence and the decrement.
    End(u8),
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Drawer {
    pub at: At,
    /// Draws it tries after this one.
    pub left: u8,
    /// Its stores issued and not yet drained by its fence.
    pub pending: bool,
}

impl Drawer {
    /// Past its second check and not finished: counted, or stores in flight.
    pub fn drawing(&self) -> bool {
        matches!(self.at, At::Write | At::End(_)) || self.pending
    }
}
