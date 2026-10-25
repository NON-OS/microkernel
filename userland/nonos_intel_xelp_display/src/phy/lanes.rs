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
//! Which lanes a link uses: the loadgen setting each lane takes (Vol 12
//! p. 392, step 2) and the lanes the PHY powers down (PORT_CL_DW10 Static
//! Power Down, 7:4, Vol 2c p. 888). Lane reversal is not offered.

use nonos_intel_display_hw::access::{rmw, Mmio};
use nonos_intel_display_hw::error::Error;

use crate::regs::clka::Combo;
use crate::regs::phy::{self, port};

/// Loadgen Select for lanes 0 to 3: at most 6 Gbit/s on 4 lanes 0, 1, 1,
/// 1; at most 6 Gbit/s on 1 or 2 lanes 0, 1, 1, 0; above 6 Gbit/s none.
/// None for a lane count other than 1, 2 or 4.
pub fn loadgen(mbps: u32, lanes: u32) -> Option<[bool; 4]> {
    let fast = mbps > 6000;
    match lanes {
        4 => Some([false, !fast, !fast, !fast]),
        1 | 2 => Some([false, !fast, !fast, false]),
        _ => None,
    }
}

/// Static Power Down for `lanes` from lane 0: 0000b x4, 1100b x2, 1110b
/// x1 (p. 888).
pub fn static_power_down(lanes: u32) -> Option<u32> {
    match lanes {
        4 => Some(0b0000),
        2 => Some(0b1100),
        1 => Some(0b1110),
        _ => None,
    }
}

/// Powers up the lanes a link of `lanes` uses and down the rest (Vol 12
/// p. 144, DisplayPort enable step 7e).
pub fn power_lanes(m: &mut dyn Mmio, p: Combo, lanes: u32) -> Result<(), Error> {
    let down = static_power_down(lanes).ok_or(Error::Argument)?;
    rmw(m, port(p, phy::CL_DW10), 0xF << 4, down << 4);
    Ok(())
}
