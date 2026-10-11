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

//! The PHY at address 1 on MAC_MI_COM, on its own thread. AUX_CTRL keeps
//! eight shadows, read by selecting one through shadow 7 without the write
//! enable (tg3.h); a BMCR reset leaves autoneg on at 1000 full (0x1140).

use std::sync::{Arc, Mutex};

use nonos_devmodel::FakeBar;

use crate::constants::mi::*;
use crate::constants::phy::*;

use super::model::stable32;

#[derive(Default)]
pub struct Phy {
    pub regs: [u32; 32],
    pub shadows: [u32; 8],
    select: usize,
    pub writes: Vec<(u32, u32)>, // (register, value), in order
}

impl Phy {
    fn read(&self, reg: u32) -> u32 {
        if reg == MII_TG3_AUX_CTRL {
            return self.shadows[self.select];
        }
        self.regs[reg as usize]
    }

    fn write(&mut self, reg: u32, value: u32) {
        self.writes.push((reg, value));
        match reg {
            MII_BMCR => self.regs[0] = if value & BMCR_RESET != 0 { 0x1140 } else { value },
            MII_TG3_AUX_CTRL if value & 7 == 7 && value & MII_TG3_AUXCTL_MISC_WREN == 0 => {
                self.select = (value >> MII_TG3_AUXCTL_MISC_RDSEL_SHIFT) as usize & 7;
            }
            MII_TG3_AUX_CTRL => self.shadows[value as usize & 7] = value,
            _ => self.regs[reg as usize] = value,
        }
    }
}

pub fn phy(state: Arc<Mutex<Phy>>) -> impl Fn(&FakeBar) + Send {
    move |b| {
        let Some(com) = stable32(b, MAC_MI_COM) else { return };
        if com & MI_COM_BUSY == 0 {
            return;
        }
        let reg = (com & MI_COM_REG_ADDR_MASK) >> MI_COM_REG_ADDR_SHIFT;
        let mut p = state.lock().unwrap();
        let data = if com & MI_COM_CMD_READ != 0 {
            p.read(reg)
        } else {
            p.write(reg, com & MI_COM_DATA_MASK);
            0
        };
        b.present32(MAC_MI_COM, (com & !(MI_COM_BUSY | MI_COM_DATA_MASK)) | data);
    }
}
