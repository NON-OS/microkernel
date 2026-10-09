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

//! The PHY behind MDIC: registers by (address, page, number), the page each
//! address has selected through register 0x1F, and every command word seen.
//! A silent PHY ends each transaction in MDIC.ERROR, as one that is not
//! driving MDIO does.

use std::collections::HashMap;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Mutex;

use crate::constants::phy::{
    IGP_PAGE_SHIFT, MAX_PHY_MULTI_PAGE_REG, MDIC_DATA_MASK, MDIC_ERROR, MDIC_OP_READ,
    MDIC_OP_WRITE, MDIC_PHY_MASK, MDIC_PHY_SHIFT, MDIC_READY, MDIC_REG_MASK, MDIC_REG_SHIFT,
    PHY_PAGE_SELECT,
};

#[derive(Default)]
pub struct Phy {
    pub regs: Mutex<HashMap<(u32, u16, u32), u16>>,
    pages: Mutex<HashMap<u32, u16>>,
    pub seen: Mutex<Vec<u32>>,
    pub silent: AtomicBool,
    /// Transactions still to fail before the PHY answers.
    pub fail_next: AtomicU32,
}

impl Phy {
    pub fn get(&self, addr: u32, page: u16, reg: u32) -> u16 {
        *self.regs.lock().unwrap().get(&(addr, page, reg)).unwrap_or(&0)
    }

    pub fn set(&self, addr: u32, page: u16, reg: u32, v: u16) {
        self.regs.lock().unwrap().insert((addr, page, reg), v);
    }

    /// Run the command word `cmd` and answer the finished MDIC word.
    pub fn run(&self, cmd: u32) -> u32 {
        self.seen.lock().unwrap().push(cmd);
        let failing = self.fail_next.fetch_update(SeqCst, SeqCst, |n| n.checked_sub(1)).is_ok();
        if failing || self.silent.load(SeqCst) {
            return cmd | MDIC_READY | MDIC_ERROR;
        }
        let addr = (cmd & MDIC_PHY_MASK) >> MDIC_PHY_SHIFT;
        let reg = (cmd & MDIC_REG_MASK) >> MDIC_REG_SHIFT;
        let data = (cmd & MDIC_DATA_MASK) as u16;
        if cmd & MDIC_OP_WRITE != 0 && reg == PHY_PAGE_SELECT {
            self.pages.lock().unwrap().insert(addr, data >> IGP_PAGE_SHIFT);
            return cmd | MDIC_READY;
        }
        let paged = reg > MAX_PHY_MULTI_PAGE_REG;
        let page = if paged { *self.pages.lock().unwrap().get(&addr).unwrap_or(&0) } else { 0 };
        if cmd & MDIC_OP_WRITE != 0 {
            self.set(addr, page, reg, data);
            return cmd | MDIC_READY;
        }
        assert_ne!(cmd & MDIC_OP_READ, 0, "an MDIC command is a read or a write");
        (cmd & !MDIC_DATA_MASK) | MDIC_READY | self.get(addr, page, reg) as u32
    }
}
