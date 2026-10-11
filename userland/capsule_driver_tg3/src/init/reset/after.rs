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

//! After the reset: the memory arbiter on, the bootcode waited for, the
//! swap modes and a quiet MAC back, MDIO without auto-polling, and whether
//! ASF firmware came back up (the rest of tg3_chip_reset).

use crate::chip::sram;
use crate::constants::chip::CHIPREV_ID_57765_A0;
use crate::constants::grc::{GRC_MODE, GRC_MODE_BASE, MEMARB_MODE, MEMARB_MODE_ENABLE};
use crate::constants::mac::MAC_MODE;
use crate::constants::mi::{MAC_MI_MODE, MAC_MI_MODE_AUTO_POLL};
use crate::constants::sram::*;
use crate::constants::timeouts::POLL_FW_MS;
use crate::init::wait;
use crate::log::Line;
use crate::setup::Driver;

/// tg3_poll_fw: the 57765 A0 needs a little more time after the bootcode.
const A0_SETTLE_MS: u64 = 10;

pub fn run(d: &mut Driver) -> Result<(), &'static str> {
    let regs = d.regs;
    // SAFETY: `regs` carries the broker MmioMap base for BAR0 and every
    // offset written here is a 32-bit register inside it.
    unsafe { regs.w32(MEMARB_MODE, MEMARB_MODE_ENABLE) };
    let done = !NIC_SRAM_FIRMWARE_MBOX_MAGIC1;
    // Not an error: a board without bootcode never answers (tg3_poll_fw).
    if !wait::until(POLL_FW_MS, || sram::read(&regs, NIC_SRAM_FIRMWARE_MBOX) == done) {
        Line::new().text("no firmware running").send();
    }
    if d.chip.rev == CHIPREV_ID_57765_A0 {
        wait::hold(A0_SETTLE_MS);
    }
    d.modes.mi_mode &= !MAC_MI_MODE_AUTO_POLL;
    unsafe {
        regs.w32(GRC_MODE, GRC_MODE_BASE);
        regs.w32f(MAC_MODE, 0);
        wait::hold(1);
        regs.w32f(MAC_MI_MODE, d.modes.mi_mode);
        wait::hold(1);
    }
    d.modes.asf = sram::read(&regs, NIC_SRAM_DATA_SIG) == NIC_SRAM_DATA_SIG_MAGIC
        && sram::read(&regs, NIC_SRAM_DATA_CFG) & NIC_SRAM_DATA_CFG_ASF_ENABLE != 0;
    Ok(())
}
