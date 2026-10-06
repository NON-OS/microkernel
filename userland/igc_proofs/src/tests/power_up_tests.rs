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

//! The PHY power-up under the PHY semaphore: a PHY left powered down comes
//! back with autonegotiation restarted, the semaphore goes back either way,
//! and a PHY firmware will not give up is never touched.

use std::sync::atomic::Ordering;

use crate::constants::phy::{SWFW_PHY0_FW, SWFW_PHY0_SW, SWSM_SMBI, SWSM_SWESMBI};
use crate::constants::regs::{REG_MDIC, REG_SWSM, REG_SW_FW_SYNC};
use crate::init::phy::mdic::READ_SLOW;
use crate::init::phy::power_up::run;
use crate::init::phy::swfw::PHY_HELD;

use super::model::{live, regs, window};
use super::phy_model::{bmcr, phy, BMCR_POWERED_DOWN};

#[test]
fn a_powered_down_phy_is_powered_up_and_the_semaphore_given_back() {
    let bar = window();
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, phy(held.clone()));
    assert_eq!(run(&regs(&bar)), Ok(()));
    assert_eq!(held.load(Ordering::SeqCst), 0x1340, "on, AN enabled and restarted");
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC) & SWFW_PHY0_SW, 0);
    assert_eq!(bar.wrote32(REG_SWSM) & (SWSM_SMBI | SWSM_SWESMBI), 0);
}

#[test]
fn a_silent_phy_still_has_its_semaphore_given_back() {
    let bar = window();
    assert_eq!(run(&regs(&bar)), Err(READ_SLOW));
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC) & SWFW_PHY0_SW, 0, "released after failure");
}

#[test]
fn a_phy_firmware_keeps_is_never_written() {
    let bar = window();
    bar.present32(REG_SW_FW_SYNC, SWFW_PHY0_FW);
    assert_eq!(run(&regs(&bar)), Err(PHY_HELD));
    assert_eq!(bar.wrote32(REG_MDIC), 0, "no MDIC cycle without the PHY bit");
}
