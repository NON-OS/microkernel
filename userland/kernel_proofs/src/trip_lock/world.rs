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

//! The world the trip lock model searches: H (writer) and R (reader), C
//! (writer) and D (reader); kills, parking, a stop, nested reads, a reuse.

pub(super) const NOW: u64 = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum P {
    Out,
    Waiting,
    In,
    Asleep,
    Done,
    Parked,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Knob {
    Built,
    NoSweep,
    EndedOnly,
    OffCpuOnly,
    NoNextSweep,
    NoForget,
    NeverStale,
    NoNestedRead,
    NoWakeList,
    NoLetGo,
    NoPutRight,
    ForgetKeepsWaiting,
    NoNextSweepNeverStale,
}

pub(super) const WHOLE: [bool; 4] = [true, false, true, false];
pub(super) fn pid(i: usize) -> u32 {
    10 + i as u32
}
pub(super) use super::world_state::World;
