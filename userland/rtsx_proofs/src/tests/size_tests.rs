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

//! A card larger than a 32-bit argument reaches is refused at
//! identification, so no block of it is ever addressed by a value that
//! wrapped: an SDUC card past 2 TiB (CMD22 is not sent), and a byte
//! addressed card whose CSD claims more than 4 GiB.

use super::rig::{sdhc, up};
use crate::card::{bring_up, power_off};
use crate::error::RtsxError;
use crate::fields::put;
use crate::model::{with, Card};
use crate::regs::host::{BIPR, SD_INT};

#[test]
fn a_card_past_what_a_32_bit_argument_reaches_is_refused() {
    let mut sduc = sdhc(1);
    sduc.csd = [0; 4];
    put(&mut sduc.csd, 126, 2, 2);
    put(&mut sduc.csd, 48, 28, 0x0FFF_FFFF);
    let mut drv = up(Some(sduc));
    assert_eq!(bring_up(&mut drv), Err(RtsxError::TooLarge));
    // A version 1 CSD naming 32 KiB blocks: 64 GiB, byte addressed. The
    // slot is powered down and its latch taken, as the watcher does.
    let mut hostile = Card::sdsc(4095, 2);
    put(&mut hostile.csd, 80, 4, 15);
    assert!(power_off(&mut drv).is_ok());
    with(|r| r.pull().map(|_| r.insert(hostile)));
    drv.regs.w32(BIPR, SD_INT);
    assert_eq!(bring_up(&mut drv), Err(RtsxError::TooLarge));
}
