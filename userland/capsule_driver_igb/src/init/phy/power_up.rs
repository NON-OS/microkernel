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
//! igb_power_up_phy_copper: firmware or a previous OS can leave the PHY
//! powered down (Linux igb_power_down_link does when the interface is not
//! running), and then no link ever comes. BMCR.POWER_DOWN is cleared, and in
//! the same write AUTO_NEG_EN and RESTART_AUTO_NEG are set, the restart
//! igb_copper_link_autoneg ends with.
//!
//! The advertisement registers are left as the PHY holds them. Linux writes
//! them to its default (10/100/1000) first; this driver does not, so whatever
//! the PHY holds applies: its own defaults after power-on, or what a previous
//! OS last wrote.
//!
//! On the I210 and I211 the PHY is paged, and BMCR is register 0 of page 0
//! only: igb_read_phy_reg_gs40g writes the page before each access, and so
//! does this. The PHY bit of SW_FW_SYNC is held across every access, and
//! goes back whether or not they worked, or firmware stays locked out.

use crate::constants::part::Part;
use crate::constants::phy::{
    GS40G_PAGE_SELECT, MII_CR_AUTO_NEG_EN, MII_CR_POWER_DOWN, MII_CR_RESTART_AUTO_NEG, PHY_CONTROL,
};
use crate::regs::Regs;

use super::{mdic, swfw};

pub fn bmcr_on(bmcr: u16) -> u16 {
    (bmcr & !MII_CR_POWER_DOWN) | MII_CR_AUTO_NEG_EN | MII_CR_RESTART_AUTO_NEG
}

pub fn run(regs: &Regs, part: Part) -> Result<(), &'static str> {
    swfw::acquire(regs)?;
    let page = || match part.paged_phy() {
        true => mdic::write(regs, GS40G_PAGE_SELECT, 0),
        false => Ok(()),
    };
    let result = page()
        .and_then(|()| mdic::read(regs, PHY_CONTROL))
        .and_then(|bmcr| page().and_then(|()| mdic::write(regs, PHY_CONTROL, bmcr_on(bmcr))));
    let released = swfw::release(regs);
    result.and(released)
}
