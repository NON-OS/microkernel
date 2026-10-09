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

//! The parts of Linux r8153_hw_phy_cfg that shape the signal, on the
//! RTL8153 (RTL_VER_03 to 06): the line impedance, the low-pass filter
//! corner auto tune and the 10 Mb/s amplitude, in the PHY SRAM. Then
//! pause frames are advertised (r8152b_enable_fc), which both
//! r8153_hw_phy_cfg and r8153b_hw_phy_cfg end with.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{mdio_read, mdio_write, sram_write, Dev};
use crate::r8153::regs::mii::{ADVERTISE_PAUSE_ASYM, ADVERTISE_PAUSE_CAP, MII_ADVERTISE};
use crate::r8153::regs::phy::{SRAM_10M_AMP1, SRAM_10M_AMP2, SRAM_IMPEDANCE, SRAM_LPF_CFG};
use crate::r8153::Version;

/// The values r8153_hw_phy_cfg writes, in its order.
const SRAM_TUNING: [(u16, u16); 4] = [
    (SRAM_IMPEDANCE, 0x0b13),
    (SRAM_LPF_CFG, 0xf70f),
    (SRAM_10M_AMP1, 0x00af),
    (SRAM_10M_AMP2, 0x0208),
];

pub fn phy_cfg<B: Bus>(dev: &mut Dev<B>, v: Version) -> Result<(), Fail> {
    if !v.is_8153b() {
        for (addr, value) in SRAM_TUNING {
            at("PHY SRAM write refused", sram_write(dev, addr, value))?;
        }
    }
    let anar = at("PHY ANAR unread", mdio_read(dev, MII_ADVERTISE))?;
    let pause = anar | ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM;
    at("pause frames not advertised", mdio_write(dev, MII_ADVERTISE, pause))
}
