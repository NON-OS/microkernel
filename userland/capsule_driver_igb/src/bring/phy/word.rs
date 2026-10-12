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

//! MDIC words: data in bits 15:0, register in 20:16, PHY address in 25:21,
//! the opcode, then READY and ERROR, which the part sets when the cycle
//! ends (facts::phy).

use crate::facts::phy::{
    MAX_PHY_REG, MDIC_DATA_MASK, MDIC_ERROR, MDIC_OP_READ, MDIC_OP_WRITE, MDIC_PHY_SHIFT,
    MDIC_READY, MDIC_REG_SHIFT, PHY_ADDR,
};

pub const REG_RANGE: &str = "phy register beyond MAX_PHY_REG_ADDRESS";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mdic {
    Busy,
    Done(u16),
    Failed,
}

/// A register past clause 22's five address bits is refused, not masked:
/// masking would reach a different register.
fn address(reg: u32) -> Result<u32, &'static str> {
    if reg > MAX_PHY_REG {
        return Err(REG_RANGE);
    }
    Ok((reg << MDIC_REG_SHIFT) | (PHY_ADDR << MDIC_PHY_SHIFT))
}

pub fn read_cmd(reg: u32) -> Result<u32, &'static str> {
    Ok(address(reg)? | MDIC_OP_READ)
}

pub fn write_cmd(reg: u32, data: u16) -> Result<u32, &'static str> {
    Ok(address(reg)? | MDIC_OP_WRITE | (data as u32 & MDIC_DATA_MASK))
}

/// READY first: ERROR is only meaningful once the cycle has ended.
pub fn outcome(mdic: u32) -> Mdic {
    if mdic & MDIC_READY == 0 {
        Mdic::Busy
    } else if mdic & MDIC_ERROR != 0 {
        Mdic::Failed
    } else {
        Mdic::Done((mdic & MDIC_DATA_MASK) as u16)
    }
}
