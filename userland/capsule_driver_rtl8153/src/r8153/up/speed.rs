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

//! Autonegotiation of every speed the RTL8153 has, as Linux
//! rtl8152_set_speed with AUTONEG_ENABLE and the probe's advertising
//! (10 and 100 half and full, 1000 full): the advertisement registers,
//! then BMCR with negotiation restarted and, because hw_phy_cfg set
//! PHY_RESET, a PHY reset, which is waited out.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{mdio_read, mdio_write, wait_until, Dev};
use crate::r8153::regs::mii::{ADVERTISE_1000FULL, ADVERTISE_1000HALF, ADVERTISE_100FULL};
use crate::r8153::regs::mii::{ADVERTISE_100HALF, ADVERTISE_10FULL, ADVERTISE_10HALF};
use crate::r8153::regs::mii::{BMCR_ANENABLE, BMCR_ANRESTART, BMCR_RESET};
use crate::r8153::regs::mii::{MII_ADVERTISE, MII_BMCR, MII_CTRL1000};

const ADVERTISE_ALL: u16 =
    ADVERTISE_10HALF | ADVERTISE_10FULL | ADVERTISE_100HALF | ADVERTISE_100FULL;
/// Linux waits 50 passes of 20 ms for the reset bit to clear.
const RESET_MS: u64 = 1_000;

pub fn autoneg<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    let anar = at("PHY ANAR unread", mdio_read(dev, MII_ADVERTISE))?;
    if anar | ADVERTISE_ALL != anar {
        at("PHY ANAR refused", mdio_write(dev, MII_ADVERTISE, anar | ADVERTISE_ALL))?;
    }
    let gbcr = at("PHY 1000BASE-T control unread", mdio_read(dev, MII_CTRL1000))?;
    let want = (gbcr & !ADVERTISE_1000HALF) | ADVERTISE_1000FULL;
    if want != gbcr {
        at("PHY 1000BASE-T control refused", mdio_write(dev, MII_CTRL1000, want))?;
    }
    let bmcr = BMCR_ANENABLE | BMCR_ANRESTART | BMCR_RESET;
    at("negotiation not started", mdio_write(dev, MII_BMCR, bmcr))?;
    let reset_done = |d: &mut Dev<B>| Ok(mdio_read(d, MII_BMCR)? & BMCR_RESET == 0);
    at("PHY reset not done", wait_until(dev, RESET_MS, 20, reset_done))
}
