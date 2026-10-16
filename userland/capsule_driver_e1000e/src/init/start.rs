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

//! A reset part put on the air: the station address and filter, the link,
//! both rings named, Bus Master on, then the units, then the silicon errata.
//! Bus Master goes on after the rings so a part never masters while its ring
//! registers name a previous owner's frames; the units go on after it so
//! neither starts a fetch the bus would refuse. From then on every
//! descriptor the part holds is this driver's (`on_own_rings`).

use crate::constants::regs::{REG_RCTL, REG_TCTL};
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::setup::{command, Driver};

use super::run::warn;
use super::{errata, link, mac_filter, rx_setup, tx_setup};

pub fn run(d: &mut Driver) -> Result<(), &'static str> {
    mac_filter::program(&d.regs, &d.mac);
    // Linux goes on when the PHY refuses the advertisement; so does this:
    // the PHY restarts autonegotiation by itself after its reset.
    warn(link::run(&d.regs, d.family).err());
    rx_setup::program(&d.regs, &d.rx, d.rx_ring_device_addr);
    tx_setup::program(&d.regs, &d.tx, d.tx_ring_device_addr);
    command(d.device_id, d.claim_epoch, true)?;
    d.on_own_rings = true;
    // SAFETY: `d.regs` is the broker-mapped BAR0 window; RCTL and TCTL are
    // 4-byte registers inside it on every family.
    unsafe {
        d.regs.modify(REG_RCTL, 0, RCTL_EN);
        d.regs.modify(REG_TCTL, 0, TCTL_EN);
    }
    errata::run(&d.regs, d.family);
    Ok(())
}
