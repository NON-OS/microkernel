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

use crate::chip::MacVersion;
use crate::constants::regs::{CMD_RESET, CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD};
use crate::hw::{quiesce, wait_for};
use crate::log::Line;
use crate::regs::Regs;

/// Linux rtl_hw_reset polls 100 times 100 us apart for CmdReset to clear.
const RESET_MS: u64 = 10;

/// The bring-up's reset: `part`, with a part that never comes out of it
/// named on the console.
pub fn run(regs: &Regs, ver: MacVersion) -> Result<(), &'static str> {
    let done = part(regs, ver);
    if done == Err("rtl8169 reset timeout") {
        Line::new("rtl8169: reset did not complete in 10 ms, not started").send();
    }
    done
}

/// Stop the part the way its version needs, then reset it on the clock and
/// hold it to both engines off, as the reset leaves them. A teardown runs
/// this alone, and says nothing of its own.
pub fn part(regs: &Regs, ver: MacVersion) -> Result<(), &'static str> {
    quiesce(regs, ver);
    // SAFETY: ChipCmd (0x37) lies inside every mapped window, and the driver
    // owns the part.
    unsafe { regs.w8(REG_CMD, CMD_RESET) };
    // SAFETY: as above.
    let done = wait_for(RESET_MS, false, || unsafe { regs.r8(REG_CMD) } & CMD_RESET != 0);
    if !done {
        return Err("rtl8169 reset timeout");
    }
    // SAFETY: as above.
    if unsafe { regs.r8(REG_CMD) } & (CMD_RX_ENABLE | CMD_TX_ENABLE) != 0 {
        return Err("rtl8169 engines still on after reset");
    }
    Ok(())
}
