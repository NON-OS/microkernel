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

//! The slot: BIPR's view of it, a card put in or pulled out (each latches
//! the card-detect change, SD_INT), and the card's power.

use nonos_libc::note;

use super::card::Card;
use super::linux::{SD_INT, SD_WRITE_PROTECT};
use super::reader::Reader;
use crate::regs::card::{CARD_PWR_CTL, SD_POWER_MASK, SD_POWER_OFF, SD_POWER_ON};
use crate::regs::host::SD_EXIST;

impl Reader {
    pub(super) fn bipr(&self) -> u32 {
        let exist = self.slot.as_ref().map_or(0, |_| SD_EXIST);
        let wp = self.slot.as_ref().map_or(0, |c| if c.wp { SD_WRITE_PROTECT } else { 0 });
        self.pending | exist | wp
    }

    /// `card` into the slot, unpowered.
    pub fn insert(&mut self, mut card: Card) {
        card.reset();
        self.slot = Some(card);
        self.pending |= SD_INT;
    }

    /// The card out of the slot, mid-whatever.
    pub fn pull(&mut self) -> Option<Card> {
        self.pending |= SD_INT;
        self.slot.take()
    }

    pub fn powered(&self) -> bool {
        self.reg(CARD_PWR_CTL) & SD_POWER_MASK == SD_POWER_ON
    }

    pub(super) fn power_off_slot(&mut self) {
        self.regs.insert(CARD_PWR_CTL, SD_POWER_OFF);
        if let Some(card) = self.slot.as_mut() {
            card.reset();
        }
        note("card power off");
    }
}
