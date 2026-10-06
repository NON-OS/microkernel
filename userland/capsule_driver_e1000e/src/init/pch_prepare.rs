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

//! Getting the PCH PHY to answer on PCIe before the reset, in the order of
//! Linux e1000_init_phy_workarounds_pchlan: gate the hardware's own PHY
//! configuration, leave ULP, check the MAC-PHY link with the software flag
//! held, then reset the PHY into a known state.

use crate::constants::regs::REG_EXTCNF_CTRL;
use crate::constants::status::EXTCNF_CTRL_GATE_PHY_CFG;
use crate::log::Line;
use crate::setup::Driver;
use crate::swflag;

use super::{interconnect, phy_reset, ulp};

pub fn run(d: &Driver) -> Result<(), &'static str> {
    let (regs, family) = (&d.regs, d.family);
    // SAFETY: `regs` is the broker-mapped BAR0 window; EXTCNF_CTRL is a
    // 4-byte register inside it. The gate stays until the MAC reset clears
    // it, as in Linux for every family after pch2lan.
    unsafe { regs.modify(REG_EXTCNF_CTRL, 0, EXTCNF_CTRL_GATE_PHY_CFG) };
    if let Err(e) = ulp::disable(d) {
        // Linux warns "Failed to disable ULP" and goes on to the link check,
        // which is what decides whether the PHY can be reached.
        Line::new().text("ULP exit failed (").text(e).text("), continuing").send();
    }
    swflag::acquire(regs, family)?;
    let link = interconnect::run(regs, family);
    swflag::release(regs);
    link?;
    phy_reset::run(regs, family)
}
