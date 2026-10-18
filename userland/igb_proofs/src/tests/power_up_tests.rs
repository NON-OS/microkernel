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

//! The PHY power-up under this port's PHY semaphore: a PHY left powered down
//! comes back with autonegotiation restarted, the semaphore goes back either
//! way, and a PHY firmware will not give up is never touched.

use std::sync::atomic::Ordering;

use crate::bring::phy::cycle::READ_SLOW;
use crate::bring::phy::owner::PHY_HELD;
use crate::bring::phy::wake::run;
use crate::facts::parts::Part;
use crate::facts::phy::{SWSM_SMBI, SWSM_SWESMBI};
use crate::facts::regs::{REG_MDIC, REG_STATUS, REG_SWSM, REG_SW_FW_SYNC};

use super::model::{live, regs, window};
use super::phy_model::{bmcr, phy, BMCR_POWERED_DOWN};

#[test]
fn a_powered_down_phy_is_powered_up_and_the_semaphore_given_back() {
    let bar = window();
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, phy(held.clone()));
    assert_eq!(run(&regs(&bar), Part::I82576), Ok(()));
    assert_eq!(held.load(Ordering::SeqCst), 0x1340, "on, AN enabled and restarted");
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC), 0, "PHY0 bit given back");
    assert_eq!(bar.wrote32(REG_SWSM) & (SWSM_SMBI | SWSM_SWESMBI), 0);
}

#[test]
fn a_silent_phy_still_has_its_semaphore_given_back() {
    let bar = window();
    assert_eq!(run(&regs(&bar), Part::I350), Err(READ_SLOW));
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC), 0, "released after failure");
}

/// The second port of a dual-port card (STATUS function 1) waits on PHY1,
/// not on the PHY0 firmware holds for the first port.
#[test]
fn each_port_waits_on_its_own_phy_bit() {
    let bar = window();
    bar.present32(REG_STATUS, 1 << 2);
    bar.present32(REG_SW_FW_SYNC, 0x2 << 16);
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, phy(held.clone()));
    assert_eq!(run(&regs(&bar), Part::I82576), Ok(()));
    assert_eq!(held.load(Ordering::SeqCst), 0x1340);
    assert_eq!(bar.wrote32(REG_SW_FW_SYNC), 0x2 << 16, "firmware's PHY0 hold untouched");
}

#[test]
fn a_phy_firmware_keeps_is_never_written() {
    let bar = window();
    bar.present32(REG_SW_FW_SYNC, 0x2 << 16);
    assert_eq!(run(&regs(&bar), Part::I82576), Err(PHY_HELD));
    assert_eq!(bar.wrote32(REG_MDIC), 0, "no MDIC cycle without the PHY bit");
}
