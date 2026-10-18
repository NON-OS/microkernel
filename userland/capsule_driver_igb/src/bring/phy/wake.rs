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
//! Waking the PHY. Firmware or a previous OS may leave it powered down, and
//! then no link ever comes. BMCR.POWER_DOWN is cleared, and in the same
//! write autonegotiation is enabled and restarted, so the link comes up
//! with whatever the PHY can offer.
//!
//! The advertisement registers are left as the PHY holds them: its own
//! defaults after power-on, or what a previous OS last wrote. This driver
//! has no reason to advertise less than the PHY does.
//!
//! On the I210 and I211 the PHY is paged and BMCR is register 0 of page 0
//! only, so page 0 is selected before each access. This port's PHY lock is
//! held across every access, and goes back whether or not they worked, or
//! firmware stays locked out.

use crate::facts::parts::Part;
use crate::facts::phy::{
    GS40G_PAGE_SELECT, MII_CR_AUTO_NEG_EN, MII_CR_POWER_DOWN, MII_CR_RESTART_AUTO_NEG, PHY_CONTROL,
};
use crate::regs::Regs;

use super::{cycle, owner};

pub fn bmcr_on(bmcr: u16) -> u16 {
    (bmcr & !MII_CR_POWER_DOWN) | MII_CR_AUTO_NEG_EN | MII_CR_RESTART_AUTO_NEG
}

pub fn run(regs: &Regs, part: Part) -> Result<(), &'static str> {
    owner::acquire(regs)?;
    let page = || match part.paged_phy() {
        true => cycle::write(regs, GS40G_PAGE_SELECT, 0),
        false => Ok(()),
    };
    let result = page()
        .and_then(|()| cycle::read(regs, PHY_CONTROL))
        .and_then(|bmcr| page().and_then(|()| cycle::write(regs, PHY_CONTROL, bmcr_on(bmcr))));
    let released = owner::release(regs);
    result.and(released)
}
