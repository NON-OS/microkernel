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

//! Linux e1000_toggle_lanphypc_pch_lpt: drive the LANPHYPC pin low for a
//! moment, which power-cycles the PHY and brings the MAC-PHY link back in
//! PCIe mode from wherever firmware left it.

use crate::constants::ctrl::{CTRL_EXT_LPCD, CTRL_LANPHYPC_OVERRIDE, CTRL_LANPHYPC_VALUE};
use crate::constants::pch_bits::{FEXTNVM3_PHY_CFG_COUNTER_50MSEC, FEXTNVM3_PHY_CFG_COUNTER_MASK};
use crate::constants::regs::{REG_CTRL, REG_CTRL_EXT, REG_STATUS};
use crate::constants::regs_pch::REG_FEXTNVM3;
use crate::constants::timeouts::{LANPHYPC_SETTLE_MS, LPCD_MS, SHORT_MS};
use crate::log::say;
use crate::regs::Regs;
use crate::wait::{idle_until, sleep_ms};

pub fn toggle(regs: &Regs) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; FEXTNVM3, CTRL,
    // CTRL_EXT and STATUS are 4-byte registers inside it on the PCH parts.
    let ctrl = unsafe {
        // The PHY configuration counter to 50 ms.
        regs.modify(REG_FEXTNVM3, FEXTNVM3_PHY_CFG_COUNTER_MASK, FEXTNVM3_PHY_CFG_COUNTER_50MSEC);
        let ctrl = (regs.r32(REG_CTRL) | CTRL_LANPHYPC_OVERRIDE) & !CTRL_LANPHYPC_VALUE;
        regs.w32(REG_CTRL, ctrl);
        let _ = regs.r32(REG_STATUS);
        ctrl
    };
    // Linux holds the pin 10 to 20 us; a millisecond is this clock's shortest.
    sleep_ms(SHORT_MS);
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_CTRL, ctrl & !CTRL_LANPHYPC_OVERRIDE);
        let _ = regs.r32(REG_STATUS);
    }
    // SAFETY: as above.
    let done = || unsafe { regs.r32(REG_CTRL_EXT) } & CTRL_EXT_LPCD != 0;
    if !idle_until(LPCD_MS, done) {
        // Linux waits the same and goes on without it.
        say("LANPHYPC power cycle not reported done in 130 ms, continuing");
    }
    sleep_ms(LANPHYPC_SETTLE_MS);
}
