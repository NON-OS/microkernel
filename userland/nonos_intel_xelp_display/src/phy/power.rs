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
//! A combo port's IO power, in the driver's PWR_WELL_CTL_DDI2 (Vol 12 p.
//! 143, DisplayPort enable step 5; Vol 2c pp. 1071 to 1075). Vol 12 gives
//! 20 us for the state to follow, and that is the deadline used.

use nonos_intel_display_hw::access::{rmw, Clock, Mmio, Until};
use nonos_intel_display_hw::error::Error;

use crate::regs::clka::Combo;
use crate::regs::ddi::{io_power, PWR_WELL_CTL_DDI2};
use crate::wait;

const IO_POWER_US: u64 = 20;

/// Asks for `p`'s IO power on or off and waits for its state. Vol 2c says
/// the request must not change while the state is still following an
/// earlier one (p. 1072), so a request and state that disagree are
/// refused before the write.
pub fn io_power_set(m: &mut dyn Mmio, c: &dyn Clock, p: Combo, on: bool) -> Result<(), Error> {
    let (req, state) = io_power(p);
    let now = m.read(PWR_WELL_CTL_DDI2);
    if (now & req != 0) != (now & state != 0) {
        return Err(Error::Argument);
    }
    rmw(m, PWR_WELL_CTL_DDI2, req, if on { req } else { 0 });
    let want = if on { state } else { 0 };
    wait::bits(m, PWR_WELL_CTL_DDI2, state, want, &Until::new(c, IO_POWER_US))
}
