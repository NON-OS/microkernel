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
//! The Combo PHY Initialization Sequence (Vol 12 p. 391, 392). PHY A is
//! the compensation source for the DPLLs and PHYs B and C (p. 391), so B
//! and C are refused until A is initialised.

use nonos_intel_display_hw::access::{rmw, Mmio};
use nonos_intel_display_hw::error::Error;

use super::procmon::for_dw3;
use crate::regs::clka::Combo;
use crate::regs::ddi::{phy_misc, MISC_COMP_PWR_DOWN};
use crate::regs::phy::{self, group, lane, port};

/// PORT_COMP_DW0 bit 31, Comp Init (Vol 2c p. 896).
const COMP_INIT: u32 = 1 << 31;
/// PORT_TX_DW8 bit 31 odcc_clksel, 30:29 odcc_clk_div_sel (p. 953).
const ODCC_SEL: u32 = 1 << 31;
const ODCC_DIV: u32 = 3 << 29;
/// PORT_PCS_DW1 21:20, DCC Mode Select; 11b runs it continuously (p. 906).
const DCC_MODE: u32 = 3 << 20;
/// PORT_COMP_DW8 bit 24, irefgen (p. 901).
const IREFGEN: u32 = 1 << 24;
/// PORT_CL_DW5 bit 4, CL Power Down Enable (p. 886).
const CL_POWER_DOWN: u32 = 1 << 4;

/// Initialises port `p`'s PHY, or leaves it when Comp Init is already set
/// (step 1). Everything is read and checked before the first write: a
/// PHY whose voltage and process have no procmon values, or B or C before
/// A, is refused untouched.
pub fn init(m: &mut dyn Mmio, p: Combo) -> Result<(), Error> {
    if m.read(port(p, phy::COMP_DW0)) & COMP_INIT != 0 {
        return Ok(());
    }
    if p != Combo::A && m.read(port(Combo::A, phy::COMP_DW0)) & COMP_INIT == 0 {
        return Err(Error::Argument);
    }
    let pm = for_dw3(m.read(port(p, phy::COMP_DW3))).ok_or(Error::Argument)?;
    // Step 2: the DCC clock and continuous DCC, the same on every lane, so
    // read from lane 0 and written to the group.
    let dw8 = m.read(lane(p, 0, phy::TX_DW8));
    m.write(group(p, phy::TX_DW8), (dw8 & !(ODCC_SEL | ODCC_DIV)) | ODCC_SEL);
    let pcs = m.read(lane(p, 0, phy::PCS_DW1));
    m.write(group(p, phy::PCS_DW1), pcs | DCC_MODE);
    // Step 3, then step 4's procmon values.
    rmw(m, phy_misc(p), MISC_COMP_PWR_DOWN, 0);
    rmw(m, port(p, phy::COMP_DW1), 0xFF, pm.dw1_low);
    rmw(m, port(p, phy::COMP_DW1), 0xFF << 16, pm.dw1_high << 16);
    m.write(port(p, phy::COMP_DW9), pm.dw9);
    m.write(port(p, phy::COMP_DW10), pm.dw10);
    // Steps 5 to 7.
    if p == Combo::A {
        rmw(m, port(p, phy::COMP_DW8), IREFGEN, IREFGEN);
    }
    rmw(m, port(p, phy::COMP_DW0), COMP_INIT, COMP_INIT);
    rmw(m, port(p, phy::CL_DW5), CL_POWER_DOWN, CL_POWER_DOWN);
    Ok(())
}
