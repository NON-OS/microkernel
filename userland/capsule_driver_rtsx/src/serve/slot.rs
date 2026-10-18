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

//! The slot, looked at between requests and before each one that reaches
//! the card. A card-detect change latched in BIPR is taken first: the
//! served card is checked again, and a refused one tried as a new card,
//! after the 200 ms rtsx_pci_isr leaves a change to settle. Then the card's presence moves the medium on
//! (change.rs). A reader that reads all ones leaves the medium as it is.

use super::change::next;
use super::medium::Medium;
use crate::card::present;
use crate::clock::sleep_ms;
use crate::regs::host::{BIPR, SD_INT};
use crate::setup::Driver;

pub const SETTLE_MS: u64 = 200;

pub fn look(drv: &mut Driver, medium: &mut Medium) {
    let bipr = drv.regs.r32(BIPR);
    if bipr == u32::MAX {
        return;
    }
    if bipr & SD_INT != 0 {
        drv.regs.w32(BIPR, SD_INT);
        *medium = medium.shaken();
        sleep_ms(SETTLE_MS);
    }
    let here = present(drv);
    *medium = next(drv, *medium, here);
}
