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

//! Auto-negotiation restarted, the last step of ax88179_reset, as Linux
//! mii_nway_restart does it: only when BMCR has it enabled.

use nonos_usbnet::Bus;

use super::access::{read_phy, write_phy, Step};
use super::phy_regs::{BMCR, BMCR_ANENABLE, BMCR_ANRESTART};

pub(super) fn restart_autoneg<B: Bus>(bus: &mut B) -> Result<(), Step> {
    let bmcr = read_phy(bus, BMCR, "PHY BMCR unread")?;
    if bmcr & BMCR_ANENABLE == 0 {
        return Ok(());
    }
    write_phy(bus, BMCR, bmcr | BMCR_ANRESTART, "PHY autoneg restart refused")
}
