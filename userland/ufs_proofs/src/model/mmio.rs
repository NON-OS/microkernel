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

//! The register file. HCE goes to 0 at once and back to 1 after a few
//! reads, as a host initialising; IS clears what is written to it; a write
//! to UICCMD runs the command and a doorbell runs the request, at once.

use super::sim::Model;
use crate::ufs::env::Mmio;
use crate::ufs::hci::regs::*;

impl Mmio for Model {
    fn r32(&self, off: u32) -> u32 {
        let mut h = self.host.borrow_mut();
        if off == HCE && h.regs[(HCE / 4) as usize] != 0 && h.hce_reads > 0 {
            h.hce_reads -= 1;
            return 0;
        }
        h.regs[(off / 4) as usize]
    }

    fn w32(&self, off: u32, v: u32) {
        match off {
            HCE => self.hce(v),
            IS => self.host.borrow_mut().regs[(IS / 4) as usize] &= !v,
            UICCMD => self.uic(v),
            UTRLDBR => {
                self.host.borrow_mut().regs[(UTRLDBR / 4) as usize] |= v;
                self.doorbell();
            }
            UTRLCLR => {
                let mut h = self.host.borrow_mut();
                h.clears.push(v);
                if !self.cfg.borrow().clear_never {
                    h.regs[(UTRLDBR / 4) as usize] &= v;
                }
            }
            _ => self.host.borrow_mut().regs[(off / 4) as usize] = v,
        }
    }
}

impl Model {
    fn hce(&self, v: u32) {
        let cfg = self.cfg.borrow();
        let mut h = self.host.borrow_mut();
        let i = |r: u32| (r / 4) as usize;
        if v & HCE_ENABLE == 0 {
            h.regs[i(HCE)] = 0;
            h.regs[i(HCS)] = 0;
            return;
        }
        h.enables += 1;
        if cfg.hce_never {
            return;
        }
        h.regs[i(HCE)] = HCE_ENABLE;
        h.hce_reads = cfg.hce_settle;
        let lists = if cfg.lists_never { 0 } else { HCS_UTRLRDY | HCS_UTMRLRDY };
        h.regs[i(HCS)] = HCS_UCRDY | lists;
    }
}
