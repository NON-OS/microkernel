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

//! Link setup, Linux e1000_setup_copper_link_pch_lpt / _82571 and
//! e1000_copper_link_autoneg: CTRL.SLU set, forced speed and duplex off,
//! then in the PHY advertise 10/100 half and full and 1000 full and restart
//! autonegotiation. The link comes up on the PHY's own time; nothing waits
//! for it. K1 is not turned off: Linux does so only on a user's request.

use crate::constants::ctrl::{CTRL_FRCDPX, CTRL_FRCSPD, CTRL_SLU};
use crate::constants::phy::{
    ADVERTISE_1000FULL, ADVERTISE_1000HALF, ADVERTISE_10_100, ADVERTISE_PAUSE_BOTH, BMCR_ANENABLE,
    BMCR_ANRESTART, MII_ADVERTISE, MII_BMCR, MII_CTRL1000,
};
use crate::constants::regs::REG_CTRL;
use crate::constants::Family;
use crate::phy::access::{read, write};
use crate::regs::Regs;
use crate::swflag;

pub fn run(regs: &Regs, family: Family) -> Result<(), &'static str> {
    // SAFETY: `regs` is the broker-mapped BAR0 window; CTRL is a 4-byte
    // register inside it.
    unsafe { regs.modify(REG_CTRL, CTRL_FRCSPD | CTRL_FRCDPX, CTRL_SLU) };
    swflag::acquire(regs, family)?;
    let r = autoneg(regs, family);
    swflag::release(regs);
    r
}

fn autoneg(regs: &Regs, family: Family) -> Result<(), &'static str> {
    let adv = read(regs, family, MII_ADVERTISE)?;
    let adv = (adv & !(ADVERTISE_10_100 | ADVERTISE_PAUSE_BOTH)) | ADVERTISE_10_100;
    write(regs, family, MII_ADVERTISE, adv)?;
    let gig = read(regs, family, MII_CTRL1000)?;
    let gig = (gig & !(ADVERTISE_1000FULL | ADVERTISE_1000HALF)) | ADVERTISE_1000FULL;
    write(regs, family, MII_CTRL1000, gig)?;
    let bmcr = read(regs, family, MII_BMCR)?;
    write(regs, family, MII_BMCR, bmcr | BMCR_ANENABLE | BMCR_ANRESTART)
}
