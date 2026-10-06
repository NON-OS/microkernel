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

//! The state word each line of the panel carries, and its colour.

use crate::display::ink::palette::{ACCENT, BAD, OK, WARN};

/// VERIFIED and FAILED are verdicts this loader reached itself. PRESENT is an
/// item it carries without checking, for the kernel to check. ON and OFF are
/// the platform's switches.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Verified,
    Failed,
    Absent,
    Present,
    NotMeasured,
    On,
    Off,
}

impl State {
    pub const fn word(self) -> &'static [u8] {
        match self {
            State::Verified => b"VERIFIED",
            State::Failed => b"FAILED",
            State::Absent => b"ABSENT",
            State::Present => b"PRESENT",
            State::NotMeasured => b"NOT MEASURED",
            State::On => b"ON",
            State::Off => b"OFF",
        }
    }

    pub const fn color(self) -> u32 {
        match self {
            State::Verified | State::On => OK,
            State::Failed => BAD,
            State::Absent | State::NotMeasured | State::Off => WARN,
            State::Present => ACCENT,
        }
    }
}
