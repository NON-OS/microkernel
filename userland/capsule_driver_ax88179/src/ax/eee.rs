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

//! Energy Efficient Ethernet off, as ax88179_reset leaves it: the PHY's
//! own switch (ax88179_disable_eee), then nothing advertised to the link
//! partner (ax88179_ethtool_set_eee through ax88179_phy_mmd_indirect).

use nonos_usbnet::Bus;

use super::access::{write_phy, Step};
use super::phy_regs::{AN_EEE_ADV, EEE_OFF, MMD_AN, MMD_CTRL, MMD_CTRL_NOINCR, MMD_DATA};
use super::phy_regs::{PAGE0, PAGE3, PAGE_SELECT, PHYADDR};

const STEPS: [(u16, u16, &str); 7] = [
    (PAGE_SELECT, PAGE3, "PHY page 3 select refused"),
    (PHYADDR, EEE_OFF, "PHY EEE off refused"),
    (PAGE_SELECT, PAGE0, "PHY page 0 select refused"),
    (MMD_CTRL, MMD_AN, "PHY MMD device select refused"),
    (MMD_DATA, AN_EEE_ADV, "PHY MMD register select refused"),
    (MMD_CTRL, MMD_AN | MMD_CTRL_NOINCR, "PHY MMD data mode refused"),
    (MMD_DATA, 0, "PHY EEE advertisement clear refused"),
];

pub(super) fn disable_eee<B: Bus>(bus: &mut B) -> Result<(), Step> {
    for (reg, value, what) in STEPS {
        write_phy(bus, reg, value, what)?;
    }
    Ok(())
}
