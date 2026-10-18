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

//! The software reset, on the clock. CR.RST clears itself once the part has
//! reset, and the reset leaves both engines off; a part that does neither in
//! time is named, and nothing is enabled or mastered on it.

use nonos_libc::Deadline;

use crate::constants::regs::{CMD_RESET, CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD};
use crate::pio::Pio;

/// Linux 8139too rtl8139_chip_reset polls 1000 times 10 us apart.
const RESET_MS: u64 = 10;

pub fn run(pio: &Pio) -> Result<(), &'static str> {
    pio.w8(REG_CMD, CMD_RESET)?;
    // Uptime counts whole milliseconds, so a deadline RESET_MS ahead can
    // fall due up to one early; one more is added.
    let deadline = Deadline::after_ms(RESET_MS + 1);
    loop {
        // The clock is read first, so the last look at the part always
        // follows the deadline: one that answered while this thread was
        // preempted is not reported as silent.
        let late = deadline.expired();
        let cmd = pio.r8(REG_CMD)?;
        if cmd & CMD_RESET == 0 {
            if cmd & (CMD_RX_ENABLE | CMD_TX_ENABLE) != 0 {
                return Err("rtl8139 engines still on after reset");
            }
            return Ok(());
        }
        if late {
            return Err("rtl8139 reset timeout");
        }
        core::hint::spin_loop();
    }
}
