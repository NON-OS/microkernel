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

//! A card leaving the slot, and one coming up in it.

use super::super::disk::EmmcDisk;
use super::super::env::{Clock, Log, Mmio};
use super::super::sdhci::regs::{CARD_ENABLE, INT_ENABLE, INT_STATUS};
use super::super::text::Line;
use super::Slot;

impl<M: Mmio + Clone, C: Clock + Clone, L: Log + Clone> Slot<M, C, L> {
    pub(super) fn drop_card(&mut self) {
        if let Some(mut d) = self.disk.take() {
            d.quiesce();
            self.parts.log.line(Line::new().s(b"card removed").as_bytes());
        }
        (self.tried, self.changed) = (false, true);
        self.arm();
    }

    /// One attempt, on a host reset for this card; a failure has been
    /// logged by the reset or the bring-up itself.
    pub(super) fn bring_up(&mut self) {
        match EmmcDisk::reset(self.parts.host()).and_then(|h| EmmcDisk::bring_up(h, self.data)) {
            Ok(d) => {
                d.say_part();
                (self.disk, self.tried, self.changed) = (Some(d), false, true);
            }
            Err(_) => {
                self.tried = true;
                self.arm();
            }
        }
    }

    /// A stopped host latches nothing: watch for the card leaving again.
    pub(super) fn arm(&self) {
        self.parts.io.w32(INT_STATUS, CARD_ENABLE as u32);
        self.parts.io.w32(INT_ENABLE, CARD_ENABLE as u32);
    }
}
