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
//! DDI_BUF_CTL on and off for a combo port (Vol 12, DisplayPort enable
//! steps 7f and 7g, p. 144; disable step 3, pp. 147, 148).

use nonos_intel_display_hw::access::{Clock, Mmio, Until};
use nonos_intel_display_hw::error::Error;

use crate::regs::clka::Combo;
use crate::regs::ddi::{buf_ctl, BUF_ENABLE, BUF_IDLE, BUF_WIDTH};
use crate::wait;

/// Vol 12's timeout for the buffer to leave idle (step 7g), and the wait it
/// allows instead of polling for idle when disabling (step 3e).
const ACTIVE_US: u64 = 1000;
const IDLE_US: u64 = 8;

/// DP Port Width for `lanes`: 000b x1, 001b x2, 011b x4 (Vol 2c p. 355).
fn width(lanes: u32) -> Option<u32> {
    match lanes {
        1 => Some(0b000),
        2 => Some(0b001),
        4 => Some(0b011),
        _ => None,
    }
}

/// Enables `p`'s buffer at `lanes` and waits for it to leave idle. Port
/// reversal and the training override are left off. A buffer that stays
/// idle is turned off again.
pub fn enable(m: &mut dyn Mmio, c: &dyn Clock, p: Combo, lanes: u32) -> Result<(), Error> {
    let w = width(lanes).ok_or(Error::Argument)?;
    let reg = buf_ctl(p);
    let keep = m.read(reg) & !(BUF_ENABLE | BUF_WIDTH | (1 << 29) | (1 << 16));
    m.write(reg, keep | BUF_ENABLE | (w << 1));
    let up = wait::bits(m, reg, BUF_IDLE, 0, &Until::new(c, ACTIVE_US));
    if up.is_err() {
        let _ = disable(m, c, p);
    }
    up
}

/// Disables `p`'s buffer, then waits for idle for up to 8 us. Vol 12 lets
/// that time stand in for the idle bit (step 3e), so the wait's end is not
/// an error.
pub fn disable(m: &mut dyn Mmio, c: &dyn Clock, p: Combo) -> Result<(), Error> {
    let reg = buf_ctl(p);
    let now = m.read(reg);
    m.write(reg, now & !BUF_ENABLE);
    let _ = wait::bits(m, reg, BUF_IDLE, BUF_IDLE, &Until::new(c, IDLE_US));
    Ok(())
}
