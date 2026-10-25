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
//! Starting and stopping a DPLL, Vol 12's DisplayPort Mode PLL Enable and
//! Disable Sequences (p. 177, 178), which HDMI shares with SSC off (p.
//! 180). Steps 7 and 10, the voltage switch around a frequency change, are
//! the caller's: it calls `configure`, switches the voltage if it must,
//! then `lock`. Mapping the PLL to a port is `map`.
//!
//! Vol 12 says the power state "should complete immediately", the lock
//! within 50 us and the unlock within 1 us. Each deadline here is ten
//! times that, and at least 100 us, so a coarse clock cannot cut it short.

use nonos_intel_display_hw::access::{rmw, Clock, Mmio, Until};
use nonos_intel_display_hw::error::Error;

use super::dividers::Dividers;
use crate::regs::pll::{self, Dpll, PLL_ENABLE, POWER_ENABLE, POWER_STATE, SSC_EN};
use crate::wait;

pub(super) const POWER_US: u64 = 100;

/// Steps 1 to 6: power, spread, frequency, and the read back that makes
/// sure the writes landed. Refused before any write when the dividers do
/// not fit or the PLL is enabled, since its frequency must not change then
/// (p. 177). A power state that never comes is switched off again.
pub fn configure(
    m: &mut dyn Mmio,
    c: &dyn Clock,
    d: Dpll,
    div: &Dividers,
    ssc: bool,
) -> Result<(), Error> {
    let (cr0, cr1) = (div.cfgcr0().ok_or(Error::Argument)?, div.cfgcr1().ok_or(Error::Argument)?);
    let en = pll::enable(d);
    if m.read(en) & PLL_ENABLE != 0 {
        return Err(Error::Argument);
    }
    rmw(m, en, POWER_ENABLE, POWER_ENABLE);
    if let Err(e) = wait::bits(m, en, POWER_STATE, POWER_STATE, &Until::new(c, POWER_US)) {
        rmw(m, en, POWER_ENABLE, 0);
        return Err(e);
    }
    rmw(m, pll::ssc(d), SSC_EN, if ssc { SSC_EN } else { 0 });
    m.write(pll::cfgcr0(d), cr0);
    m.write(pll::cfgcr1(d), cr1);
    let _ = m.read(pll::cfgcr0(d));
    Ok(())
}
