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

//! igc_acquire_swfw_sync_i225 / igc_release_swfw_sync_i225 for the PHY bit:
//! taken and given back under the semaphore, and refused by name when the
//! firmware keeps the PHY past the clock bound.

use std::time::Duration;

use crate::constants::phy::{SWFW_PHY0_FW, SWFW_PHY0_SW, SWSM_SMBI, SWSM_SWESMBI};
use crate::constants::regs::{REG_SWSM, REG_SW_FW_SYNC};
use crate::constants::timeouts::SWFW_SYNC_MS;
use crate::init::phy::swfw::{acquire, release, PHY_HELD};

use super::model::{regs, timed, window};

const EEPROM_SW: u32 = 0x1;

#[test]
fn the_phy_bit_is_taken_then_released_leaving_other_owners_alone() {
    let bar = window();
    bar.present32(REG_SW_FW_SYNC, EEPROM_SW);
    assert_eq!(acquire(&regs(&bar)), Ok(()));
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC), EEPROM_SW | SWFW_PHY0_SW);
    assert_eq!(bar.wrote32(REG_SWSM) & (SWSM_SMBI | SWSM_SWESMBI), 0, "semaphore put back");
    assert_eq!(release(&regs(&bar)), Ok(()));
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC), EEPROM_SW);
    assert_eq!(bar.wrote32(REG_SWSM) & (SWSM_SMBI | SWSM_SWESMBI), 0);
}

#[test]
fn firmware_holding_the_phy_is_waited_out_then_refused_by_name() {
    let bar = window();
    bar.present32(REG_SW_FW_SYNC, SWFW_PHY0_FW);
    let (r, took) = timed(|| acquire(&regs(&bar)));
    assert_eq!(r, Err(PHY_HELD));
    assert!(took >= Duration::from_millis(SWFW_SYNC_MS), "gave up after {took:?}");
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC), SWFW_PHY0_FW, "firmware's bit untouched");
    assert_eq!(bar.wrote32(REG_SWSM) & (SWSM_SMBI | SWSM_SWESMBI), 0);
}

#[test]
fn another_software_owner_of_the_phy_is_also_waited_for() {
    let bar = window();
    bar.present32(REG_SW_FW_SYNC, SWFW_PHY0_SW);
    assert_eq!(acquire(&regs(&bar)), Err(PHY_HELD));
}
