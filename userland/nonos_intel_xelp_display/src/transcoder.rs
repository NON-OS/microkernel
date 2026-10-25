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
//! Turning a transcoder on and off (TRANS_CONF, Vol 2c p. 1340 to 1342).

use nonos_intel_display_hw::access::{Clock, Mmio, Until};
use nonos_intel_display_hw::error::Error;
use nonos_intel_display_hw::pipe::Transcoder;

use crate::regs::trans::{of, CONF, CONF_ENABLE, CONF_INTERLACE, CONF_STATE};
use crate::wait;

/// Turns `t` on, progressive, keeping TRANS_CONF's other bits, and waits up
/// to two frames of `frame_us` for its state bit: Vol 2c says the enable
/// may be held back one frame (p. 1341). If the state never comes, the
/// enable bit is cleared again, so the transcoder is left off.
pub fn enable(m: &mut dyn Mmio, c: &dyn Clock, t: Transcoder, frame_us: u64) -> Result<(), Error> {
    let reg = of(CONF, t)?;
    let old = m.read(reg);
    m.write(reg, (old & !CONF_INTERLACE) | CONF_ENABLE);
    let until = Until::new(c, frame_us.saturating_mul(2));
    let lit = wait::bits(m, reg, CONF_STATE, CONF_STATE, &until);
    if lit.is_err() {
        let now = m.read(reg);
        m.write(reg, now & !CONF_ENABLE);
    }
    lit
}

/// Turns `t` off and waits for its state bit to clear, "timeout after two
/// frame times" (Vol 12, DisplayPort disable sequence, step 2e, p. 147).
pub fn disable(m: &mut dyn Mmio, c: &dyn Clock, t: Transcoder, frame_us: u64) -> Result<(), Error> {
    let reg = of(CONF, t)?;
    let old = m.read(reg);
    m.write(reg, old & !CONF_ENABLE);
    let until = Until::new(c, frame_us.saturating_mul(2));
    wait::bits(m, reg, CONF_STATE, 0, &until)
}
