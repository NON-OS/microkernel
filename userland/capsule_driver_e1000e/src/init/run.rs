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

//! The bring-up, in Linux's order (e1000_probe, e1000e_reset, init_hw):
//! quiesce, the PCH PHY made reachable, the global reset, the post-reset
//! bits, the family's hardware bits, K1 on pch_mtp and later, the station
//! address and filter, the link, then the rings and the silicon errata.

use crate::constants::Family;
use crate::log::Line;
use crate::setup::Driver;
use crate::swflag;

use super::{
    errata, hw_bits_82574, hw_bits_pch, k1, link, mac_filter, pch_prepare, post_reset, quiesce,
    reset, rx_setup, station_address, tx_setup,
};

pub fn bring_up(d: &mut Driver) -> Result<(), &'static str> {
    let family = d.family;
    quiesce::run(d);
    if family.is_pch() {
        pch_prepare::run(d)?;
    }
    reset::run(&d.regs, family)?;
    post_reset::run(&d.regs, family);
    if family.is_pch() {
        hw_bits_pch::run(&d.regs, family);
    } else {
        hw_bits_82574::run(&d.regs);
    }
    if family >= Family::PchMtp {
        let k1 = swflag::acquire(&d.regs, family).and_then(|()| {
            let r = k1::reconfigure(&d.regs, family);
            swflag::release(&d.regs);
            r
        });
        // e1000_init_hw_ich8lan returns this error, and e1000e_reset only
        // logs it: the part still moves frames without the new K1 timing.
        warn(k1.err());
    }
    /*
     * Drawn, not read out of the NVM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk.
     */
    d.mac = station_address::draw()?;
    mac_filter::program(&d.regs, &d.mac);
    // Linux goes on when the PHY refuses the advertisement; so does this:
    // the PHY restarts autonegotiation by itself after its reset.
    warn(link::run(&d.regs, family).err());
    rx_setup::program(&d.regs, &d.rx, d.rx_ring_device_addr);
    tx_setup::program(&d.regs, &d.tx, d.tx_ring_device_addr);
    errata::run(&d.regs, family);
    Ok(())
}

fn warn(e: Option<&'static str>) {
    if let Some(e) = e {
        Line::new().text(e).text(", continuing").send();
    }
}
