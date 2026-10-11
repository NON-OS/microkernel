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

//! One bring-up in Linux's order (tg3_open: tg3_power_up, tg3_init_hw,
//! tg3_reset_hw), for the 57765 class only. Interrupts stay masked; the
//! server polls the status block.

use crate::chip::fw::{self, Kind};
use crate::chip::sram;
use crate::constants::pcicfg::TG3PCI_MEM_WIN_BASE_ADDR;
use crate::constants::phy::{MII_TG3_RXR_COUNTERS, MII_TG3_TEST1, MII_TG3_TEST1_CRC_EN};
use crate::constants::sram::{NIC_SRAM_FIRMWARE_MBOX, NIC_SRAM_FIRMWARE_MBOX_MAGIC1};
use crate::constants::timeouts::POLL_FW_MS;
use crate::link::LinkState;
use crate::phy::{self, Mdio};
use crate::setup::Driver;

use super::{bufmgr, engines, filters, fixups, grc, mac_setup, power, reset, rings};
use super::{rx_filter, station_address, wait};

pub fn bring_up(d: &mut Driver) -> Result<LinkState, &'static str> {
    let regs = d.regs;
    power::to_vmain(d);
    // tg3_init_hw: bootcode from a fresh power-up may still be running.
    let booted = !NIC_SRAM_FIRMWARE_MBOX_MAGIC1;
    let _ = wait::until(POLL_FW_MS, || sram::read(&regs, NIC_SRAM_FIRMWARE_MBOX) == booted);
    // SAFETY: `regs` carries the broker MmioMap base for BAR0; the window
    // base is a 32-bit register inside it.
    unsafe { regs.w32(TG3PCI_MEM_WIN_BASE_ADDR, 0) };
    power::disable_ints(&regs);
    fw::stop(&regs, d.modes.asf);
    fw::sig_pre_reset(&regs, d.modes.asf, Kind::Init);
    let phy = Mdio { regs };
    if !d.chip.hw.keep_link {
        phy::reset::run(&phy, d.chip.hw.apd)?;
    }
    reset::chip_reset(d)?;
    fw::sig_legacy(&regs, d.modes.asf, Kind::Init);
    fixups::run(&regs, d.chip.rev);
    d.rx.init();
    grc::program(&regs, d.chip.rev);
    bufmgr::program(&regs)?;
    rings::program(d);
    /*
     * Drawn, not read from the NVRAM: the factory address names this card
     * to every network it joins. Drawn before the receiver is on, so a card
     * with no address never receives or sends.
     */
    d.mac = station_address::draw()?;
    mac_setup::program(&regs, &d.mac);
    engines::enable(d);
    rx_filter::enable(d);
    let link = phy::copper::setup(d);
    // Clear the PHY's CRC counter: enable it, then read it once.
    if let Ok(v) = phy.read(MII_TG3_TEST1) {
        let _ = phy.write(MII_TG3_TEST1, v | MII_TG3_TEST1_CRC_EN);
        let _ = phy.read(MII_TG3_RXR_COUNTERS);
    }
    filters::filters(d);
    fw::sig_post_reset(&regs, d.modes.asf, Kind::Init);
    Ok(link)
}
