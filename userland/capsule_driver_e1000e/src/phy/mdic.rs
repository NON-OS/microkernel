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

//! One MDI transaction through the MDIC register: the command word goes in,
//! the part runs the MDIO cycle and sets READY (and ERROR when the PHY did
//! not answer), with read data in the low 16 bits. Linux
//! e1000e_read_phy_reg_mdic / e1000e_write_phy_reg_mdic.

use crate::constants::phy::{
    MDIC_DATA_MASK, MDIC_ERROR, MDIC_OP_READ, MDIC_OP_WRITE, MDIC_PHY_MASK, MDIC_PHY_SHIFT,
    MDIC_READY, MDIC_REG_MASK, MDIC_REG_SHIFT,
};
use crate::constants::regs::REG_MDIC;
use crate::constants::timeouts::MDIC_MS;
use crate::regs::Regs;
use crate::wait::spin_until;

/// The command word for a read (`write` false, `data` ignored) or a write.
pub fn encode(write: bool, phy: u32, reg: u32, data: u16) -> u32 {
    let (op, data) =
        if write { (MDIC_OP_WRITE, data as u32 & MDIC_DATA_MASK) } else { (MDIC_OP_READ, 0) };
    op | ((phy << MDIC_PHY_SHIFT) & MDIC_PHY_MASK)
        | ((reg << MDIC_REG_SHIFT) & MDIC_REG_MASK)
        | data
}

/// What a finished MDIC word says. A read must come back naming the
/// register asked for, as Linux checks; a write is only READY and not ERROR.
pub fn decode(word: u32, write: bool, reg: u32) -> Result<u16, &'static str> {
    if word & MDIC_READY == 0 {
        return Err("MDIC not ready in 100 ms");
    }
    if word & MDIC_ERROR != 0 {
        return Err("MDIC error: the PHY did not answer");
    }
    if !write && (word & MDIC_REG_MASK) >> MDIC_REG_SHIFT != reg {
        return Err("MDIC answered for another register");
    }
    Ok((word & MDIC_DATA_MASK) as u16)
}

/// Run one transaction and wait for READY on the clock.
pub fn transact(
    regs: &Regs,
    write: bool,
    phy: u32,
    reg: u32,
    data: u16,
) -> Result<u16, &'static str> {
    let mut word = 0;
    // SAFETY: `regs` is the broker-mapped BAR0 window and MDIC a 4-byte
    // register inside it on every family this driver claims.
    unsafe { regs.w32(REG_MDIC, encode(write, phy, reg, data)) };
    spin_until(MDIC_MS, || {
        // SAFETY: as above.
        word = unsafe { regs.r32(REG_MDIC) };
        word & MDIC_READY != 0
    });
    decode(word, write, reg)
}
