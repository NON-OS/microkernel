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

//! One poll of a slot, and what it says: a card is up when presence says
//! so, its retention latch (where there is a sensor) is closed and, on a
//! port that reports it, its link is up. And what a slot can ask for.

use super::regs::{BUTTON_PRESSED, DLL_ACTIVE, POWER_CONTROLLER, POWER_FAULT, POWER_OFF, PRESENT};

/// The MRL sensor (slot capabilities bit 2) and its state (slot status
/// bit 5, set while the latch is open); recalled, unverified.
pub const MRL_SENSOR: u32 = 1 << 2;
pub const MRL_OPEN: u16 = 1 << 5;

/// What one poll read: slot status, the link (None when the port cannot
/// say), and whether the slot has power.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Read {
    pub status: u16,
    pub link: Option<bool>,
    pub powered: bool,
    /// The slot has a retention latch sensor and the latch is open.
    pub latch_open: bool,
}

impl Read {
    /// From the slot's registers: status, capabilities, link status where
    /// the port reports its link, and slot control.
    pub fn from_regs(status: u16, slot_cap: u32, link: Option<u16>, ctrl: u16) -> Read {
        let latch_open = slot_cap & MRL_SENSOR != 0 && status & MRL_OPEN != 0;
        let powered = slot_cap & POWER_CONTROLLER == 0 || ctrl & POWER_OFF == 0;
        Read { status, link: link.map(|l| l & DLL_ACTIVE != 0), powered, latch_open }
    }

    /// A card is up: present, its latch not open, its link not down.
    pub fn here(&self) -> bool {
        self.status & PRESENT != 0 && !self.latch_open && self.link != Some(false)
    }
    pub(super) fn faulted(&self) -> bool {
        self.status & POWER_FAULT != 0
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
    /// A power fault: what was in the slot departs, then its power goes.
    Fault,
}
