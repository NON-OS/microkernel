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

//! One MDIO frame through MAC_MI_COM (__tg3_readphy, __tg3_writephy). The
//! MAC's auto-polling is off from the first chip reset on (MAC_MI_MODE is
//! written without it), so no frame has to switch it off around itself.

use crate::constants::mi::*;
use crate::constants::timeouts::MDIO_MS;
use crate::init::wait;
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct Mdio {
    pub regs: Regs,
}

pub const NO_ANSWER: &str = "PHY did not answer on MDIO";

fn frame(reg: u32) -> u32 {
    ((TG3_PHY_MII_ADDR << MI_COM_PHY_ADDR_SHIFT) & MI_COM_PHY_ADDR_MASK)
        | ((reg << MI_COM_REG_ADDR_SHIFT) & MI_COM_REG_ADDR_MASK)
        | MI_COM_START
}

/*
 * SAFETY for the register accesses: `regs` carries the broker MmioMap base
 * for BAR0 and MAC_MI_COM is a 32-bit register inside it.
 */
impl Mdio {
    /// Start a frame and wait for BUSY to drop; the word read after it holds
    /// the data, as Linux reads it once more after the busy bit clears.
    fn run(&self, word: u32) -> Result<u32, &'static str> {
        unsafe { self.regs.w32f(MAC_MI_COM, word) };
        if !wait::until(MDIO_MS, || unsafe { self.regs.r32(MAC_MI_COM) & MI_COM_BUSY == 0 }) {
            return Err(NO_ANSWER);
        }
        Ok(unsafe { self.regs.r32(MAC_MI_COM) })
    }

    pub fn read(&self, reg: u32) -> Result<u32, &'static str> {
        Ok(self.run(frame(reg) | MI_COM_CMD_READ)? & MI_COM_DATA_MASK)
    }

    pub fn write(&self, reg: u32, value: u32) -> Result<(), &'static str> {
        self.run(frame(reg) | (value & MI_COM_DATA_MASK) | MI_COM_CMD_WRITE).map(|_| ())
    }
}
