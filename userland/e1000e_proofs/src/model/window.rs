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

//! BAR0 windows as each family shows them at power-on, and the PHYs.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::phy::Phy;
use crate::constants::ctrl::CTRL_EXT_LPCD;
use crate::constants::phy::{MII_ADVERTISE, MII_BMCR, MII_CTRL1000, MII_PHYSID1, MII_PHYSID2};
use crate::constants::regs::{REG_CTRL_EXT, REG_EECD, REG_TARC0};
use crate::constants::regs_pch::REG_FWSM;
use crate::constants::status::{EECD_AUTO_RD, FWSM_RSPCIPHY};

/// TARC0 bit 10 is QEMU's queue enable; reset leaves it set (e1000e_core.c).
pub const TARC0_RESET: u32 = 0x3 | (1 << 10);

/// An 82574: NVM auto-read already done, as after power-on.
pub fn i82574() -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x6000));
    bar.present32(REG_EECD, EECD_AUTO_RD);
    bar.present32(REG_TARC0, TARC0_RESET);
    bar
}

/// A PCH part with no ME firmware running: PHY resets allowed, and the
/// LANPHYPC power cycle reported done.
pub fn pch() -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x6000));
    bar.present32(REG_FWSM, FWSM_RSPCIPHY);
    bar.present32(REG_CTRL_EXT, CTRL_EXT_LPCD);
    bar
}

/// A PHY with an ID and the IEEE reset values, answering at `addr`.
pub fn phy_at(addr: u32) -> Phy {
    let phy = Phy::default();
    for ((page, reg), v) in [
        (MII_BMCR, 0x1140),
        (MII_PHYSID1, 0x0154),
        (MII_PHYSID2, 0x15A0),
        (MII_ADVERTISE, 0x0DE1),
        (MII_CTRL1000, 0x0300),
    ] {
        phy.set(addr, page, reg, v);
    }
    phy
}

/// A PCH PHY that ends every transaction in MDIC.ERROR.
pub fn silent_phy() -> Phy {
    let phy = phy_at(2);
    phy.silent.store(true, std::sync::atomic::Ordering::SeqCst);
    phy
}
