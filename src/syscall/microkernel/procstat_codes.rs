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

//! The small integers the wire format uses for a scheduler state and a
//! scheduling class. Readers mirror these tables; never renumber.

use crate::process::core::types::{Priority, ProcessState};

pub(super) fn state_code(s: &ProcessState) -> u8 {
    match s {
        ProcessState::New => 0,
        ProcessState::Ready => 1,
        ProcessState::Running => 2,
        ProcessState::Sleeping => 3,
        ProcessState::Stopped => 4,
        ProcessState::Zombie(_) => 5,
        ProcessState::Terminated(_) => 6,
    }
}

pub(super) fn priority_code(p: &Priority) -> u8 {
    match p {
        Priority::Idle => 0,
        Priority::Low => 1,
        Priority::Normal => 2,
        Priority::High => 3,
        Priority::RealTime => 4,
    }
}
