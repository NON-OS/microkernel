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

//! One poll of a slot, and what it says: a card is there when presence
//! says so and, on a port that reports it, its link is up. And what a
//! slot can ask for after one.

use super::regs::{BUTTON_PRESSED, PRESENT};

/// What one poll read: slot status, the link (None when the port cannot
/// say), and whether the slot has power.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Read {
    pub status: u16,
    pub link: Option<bool>,
    pub powered: bool,
}

impl Read {
    pub fn here(&self) -> bool {
        self.status & PRESENT != 0 && self.link != Some(false)
    }
    pub(super) fn card(&self) -> bool {
        self.status & PRESENT != 0
    }
    pub(super) fn pressed(&self) -> bool {
        self.status & BUTTON_PRESSED != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    Nothing,
    Probe,
    Depart,
    Eject,
    PowerOn,
    PowerOff,
}
