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
//! The port's media. igb_get_invariants_82575 reads it from CTRL_EXT's link
//! mode, which the NVM sets: copper behind the internal PHY, or 1000BASE-KX,
//! SGMII or SerDes. Only copper is driven here. The others need the PCS and
//! SFP or external-PHY paths Linux takes for them, so such a port is refused
//! by name rather than brought up without a link.

use crate::constants::ctrl::CTRL_EXT_LINK_MODE_MASK;
use crate::constants::regs::REG_CTRL_EXT;
use crate::regs::Regs;

pub const KX: &str = "1000BASE-KX port, not driven: copper only";
pub const SGMII: &str = "SGMII port, not driven: copper only";
pub const SERDES: &str = "SerDes port, not driven: copper only";

/// The refusal for a CTRL_EXT value, or None for a copper (GMII, 0) port.
pub fn refusal(ctrl_ext: u32) -> Option<&'static str> {
    match (ctrl_ext & CTRL_EXT_LINK_MODE_MASK) >> 22 {
        0 => None,
        1 => Some(KX),
        2 => Some(SGMII),
        _ => Some(SERDES),
    }
}

pub fn copper(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and
    // REG_CTRL_EXT is a 32-bit register inside it (e1000_regs.h).
    let ctrl_ext = unsafe { regs.r32(REG_CTRL_EXT) };
    refusal(ctrl_ext).map_or(Ok(()), Err)
}
