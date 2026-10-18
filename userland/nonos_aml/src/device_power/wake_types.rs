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

//! What runtime wake keeps and how arming can fail.

use super::state::DState;
use crate::exec::Fault;
use crate::namespace::Id;
use alloc::vec::Vec;

/// Turns a wake event on or off; false when it could not.
pub trait GpeSwitch {
    fn switch(&mut self, gpe: u16, on: bool) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeError {
    /// No `_PRW`, or one whose event is a GPE block device's.
    NoEvent,
    /// `_S0W` says it wakes from no deeper than this.
    TooDeep(DState),
    /// `_S0W` faulted or answered past D3cold.
    BadS0w,
    /// The event could not be turned on.
    Event,
    Fault(Fault),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm {
    pub dev: Id,
    pub gpe: u16,
    pub resources: Vec<Id>,
    pub target: DState,
    pub wakes: u32,
}

#[derive(Default)]
pub struct RuntimeWake {
    pub armed: Vec<Arm>,
}
