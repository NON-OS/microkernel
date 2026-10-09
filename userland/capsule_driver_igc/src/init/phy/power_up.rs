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

//! igc_power_up_phy_copper: firmware or a previous OS can leave the PHY
//! powered down (Linux igc_reset calls igc_power_down_phy_copper_base when
//! the interface is not running), and then no link ever comes.
//! BMCR.POWER_DOWN is cleared, and in the same write AUTO_NEG_EN and
//! RESTART_AUTO_NEG are set, the restart igc_copper_link_autoneg ends with.
//!
//! The advertisement registers are left as the PHY holds them. Linux writes
//! them to AUTONEG_ADVERTISE_SPEED_DEFAULT_2500 (10/100/1000/2500) first;
//! this driver does not, so whatever the PHY holds applies: its own
//! defaults after power-on, or what a previous OS last wrote.
//!
//! The PHY bit of SW_FW_SYNC is held across the read and the write, and
//! goes back whether or not they worked, or firmware stays locked out.

use crate::constants::phy::{
    MII_CR_AUTO_NEG_EN, MII_CR_POWER_DOWN, MII_CR_RESTART_AUTO_NEG, PHY_CONTROL,
};
use crate::regs::Regs;

use super::{mdic, swfw};

pub fn bmcr_on(bmcr: u16) -> u16 {
    (bmcr & !MII_CR_POWER_DOWN) | MII_CR_AUTO_NEG_EN | MII_CR_RESTART_AUTO_NEG
}

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    swfw::acquire(regs)?;
    let result = mdic::read(regs, PHY_CONTROL)
        .and_then(|bmcr| mdic::write(regs, PHY_CONTROL, bmcr_on(bmcr)));
    let released = swfw::release(regs);
    result.and(released)
}
