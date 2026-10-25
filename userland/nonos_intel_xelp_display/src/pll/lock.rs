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
//! A configured DPLL locked, and a DPLL stopped (Vol 12 p. 177, 178). The
//! deadlines are ten times the times Vol 12 expects, as in `sequence`.

use nonos_intel_display_hw::access::{rmw, Clock, Mmio, Until};
use nonos_intel_display_hw::error::Error;

use super::sequence::POWER_US;
use crate::regs::pll::{self, Dpll, PLL_ENABLE, PLL_LOCK, POWER_ENABLE, POWER_STATE};
use crate::wait;

const LOCK_US: u64 = 500;
const UNLOCK_US: u64 = 100;

/// Steps 8 and 9: enable and wait for lock. A PLL that does not lock is
/// taken back down whole, so nothing is left running.
pub fn lock(m: &mut dyn Mmio, c: &dyn Clock, d: Dpll) -> Result<(), Error> {
    let en = pll::enable(d);
    rmw(m, en, PLL_ENABLE, PLL_ENABLE);
    let locked = wait::bits(m, en, PLL_LOCK, PLL_LOCK, &Until::new(c, LOCK_US));
    if locked.is_err() {
        let _ = disable(m, c, d);
    }
    locked
}

/// The disable sequence's steps 3, 4, 6 and 7: enable off, lock gone,
/// power off, power state gone. Step 1, the ports' clocks, is `unmap`.
/// Every step runs even after a wait fails, and the first failure is
/// returned, so the power is always asked off.
pub fn disable(m: &mut dyn Mmio, c: &dyn Clock, d: Dpll) -> Result<(), Error> {
    let en = pll::enable(d);
    rmw(m, en, PLL_ENABLE, 0);
    let unlocked = wait::bits(m, en, PLL_LOCK, 0, &Until::new(c, UNLOCK_US));
    rmw(m, en, POWER_ENABLE, 0);
    let off = wait::bits(m, en, POWER_STATE, 0, &Until::new(c, POWER_US));
    unlocked.and(off)
}
