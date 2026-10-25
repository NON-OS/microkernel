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

//! What a request sees of the slot: the medium GEOMETRY describes, the
//! disk READ, WRITE and FLUSH go to, and whether it may be written.

use super::super::disk::EmmcDisk;
use super::super::env::{Clock, Log, Mmio};
use super::super::error::{EmmcError, EmmcResult};
use super::super::sdhci::regs::{PRESENT_STATE, PS_WRITE_ENABLED};
use super::Slot;

/// The slot as GEOMETRY describes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Media {
    /// Sectors on the card; 0 with none in the slot.
    pub sectors: u64,
    pub present: bool,
    pub removable: bool,
    pub read_only: bool,
}

impl<M: Mmio + Clone, C: Clock + Clone, L: Log + Clone> Slot<M, C, L> {
    /// What is in the slot now. The client has then seen the card, and data
    /// may go to it.
    pub fn describe(&mut self) -> Media {
        self.refresh();
        self.changed = false;
        Media {
            sectors: self.disk.as_ref().map_or(0, |d| d.capacity_sectors()),
            present: self.disk.is_some(),
            removable: self.removable,
            read_only: !self.writable(),
        }
    }

    /// The disk data goes to: none while the slot is empty, and none for a
    /// card the client has not had GEOMETRY of since it came, so no sector
    /// meant for one card is read from or written to another.
    pub fn data_disk(&mut self) -> EmmcResult<&mut EmmcDisk<M, C, L>> {
        self.refresh();
        if self.changed {
            return Err(EmmcError::NoMedium);
        }
        self.disk.as_mut().ok_or(EmmcError::NoMedium)
    }

    /// The card's write-protect switch is off. A fixed slot has none wired,
    /// so its pin is not read.
    pub fn writable(&self) -> bool {
        !self.removable || self.parts.io.r32(PRESENT_STATE) & PS_WRITE_ENABLED != 0
    }

    /// The card in the slot, for IDENTIFY: its name is the card's own,
    /// whatever GEOMETRY last said.
    pub fn card_disk(&mut self) -> EmmcResult<&EmmcDisk<M, C, L>> {
        self.refresh();
        self.disk.as_ref().ok_or(EmmcError::NoMedium)
    }
}
