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

//! A person taking the card out of a removable slot and putting one in.
//! Card detect follows; the host latches Card Removal (bit 7) or Card
//! Insertion (bit 6) when its status enable is set.

use super::super::sd::{SdCard, SdCfg};
use super::sim::Sim;

impl Sim {
    /// Nothing is left to answer: no SD card, and the eMMC model absent.
    pub fn take_card(&mut self) {
        self.cfg.cd_inserted = false;
        self.sd = None;
        self.card.cfg.absent = true;
        self.latch(7);
    }

    /// The card is powered with the bus, so it comes up idle if the bus is
    /// on and off if it is not.
    pub fn put_sd(&mut self, cfg: SdCfg) {
        self.cfg.cd_inserted = true;
        let mut card = SdCard::new(cfg);
        card.power(self.regs[0x29] & 1 != 0);
        self.sd = Some(card);
        self.latch(6);
    }

    fn latch(&mut self, bit: u32) {
        if self.get(0x34, 2) & 1 << bit != 0 {
            self.int |= 1 << bit;
        }
    }
}
