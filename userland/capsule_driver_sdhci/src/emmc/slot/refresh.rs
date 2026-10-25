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

//! A removable slot's look at its card detect, before every request.

use super::super::env::{Clock, Log, Mmio};
use super::super::sdhci::regs::{
    INT_CARD_REMOVE, INT_STATUS, PRESENT_STATE, PS_CARD_INSERTED, PS_CARD_STABLE,
};
use super::policy::{step, Step};
use super::Slot;

const SETTLED: u32 = PS_CARD_INSERTED | PS_CARD_STABLE;

impl<M: Mmio + Clone, C: Clock + Clone, L: Log + Clone> Slot<M, C, L> {
    pub fn refresh(&mut self) {
        if !self.removable {
            return;
        }
        let io = &self.parts.io;
        let inserted = io.r32(PRESENT_STATE) & SETTLED == SETTLED;
        let removed = io.r32(INT_STATUS) & INT_CARD_REMOVE as u32 != 0;
        if removed {
            io.w32(INT_STATUS, INT_CARD_REMOVE as u32);
        }
        match step(self.disk.is_some(), inserted, removed, self.tried) {
            Step::Keep => {}
            Step::Empty => self.tried = false,
            Step::Drop => self.drop_card(),
            Step::Swap => {
                self.drop_card();
                self.bring_up();
            }
            Step::BringUp => self.bring_up(),
        }
    }
}
