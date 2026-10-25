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

//! The model's steps as switches, so each rule can be undone one at a time.

/// A flag `stopped()` loads; each load is one step.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Flag {
    Held,
    Gone,
    Takers,
}

/// What a draw reads again once it is counted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recheck {
    Full,
    HeldOnly,
    Absent,
}

#[derive(Clone, Copy, Debug)]
pub struct Gate {
    /// The loads of `stopped()` in order; the first that finds a stop ends it.
    pub reads: &'static [Flag],
    /// `drawable()` asks `stopped()` before it counts.
    pub first_check: bool,
    pub recheck: Recheck,
    /// A draw stopped at its second check takes its count back.
    pub backs_out: bool,
    /// The guard's drop fences before it decrements.
    pub fence_first: bool,
    /// `stop_drawing` counts itself in TAKERS before it waits.
    pub counts_first: bool,
    /// `stop_drawing` waits until DRAWS reads 0.
    pub waits: bool,
    /// A takeover whose claim is made ready stores GONE.
    pub sets_gone: bool,
    /// `end_takeover` takes one from TAKERS.
    pub ends: bool,
    /// A takeover's end comes after the admit that latches GONE.
    pub ends_last: bool,
}
