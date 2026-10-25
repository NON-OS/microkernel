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

//! A doorbell on slot 0, served at once: the UTRD read where UTRLBA points,
//! the UPIU and PRDT read from the command descriptor, the answer written at
//! the response offset, OCS success, and the doorbell bit dropped. A request
//! the device does not answer stays in the doorbell.

use super::mem::{le32, read, write};
use super::sim::Model;
use crate::ufs::facts::hci::*;
use crate::ufs::facts::upiu::{COMMAND, NOP_OUT, QUERY_REQ};

impl Model {
    pub(super) fn doorbell(&self) {
        let (base, running) = {
            let h = self.host.borrow();
            let lo = h.regs[(UTRLBA / 4) as usize] as u64;
            let hi = h.regs[(UTRLBAU / 4) as usize] as u64;
            (hi << 32 | lo, h.regs[(UTRLRSR / 4) as usize] & RUN != 0)
        };
        if !running {
            return;
        }
        let mut d = [0u8; 32];
        unsafe { read(base, &mut d) };
        let dw0 = le32(&d, 0);
        let ucd = (le32(&d, 20) as u64) << 32 | le32(&d, 16) as u64;
        let (dw6, dw7) = (le32(&d, 24), le32(&d, 28));
        let dir = (dw0 >> 25) & 3;
        self.host.borrow_mut().utrds.push((dw0 >> 28, dir));
        let mut upiu = [0u8; 32];
        unsafe { read(ucd, &mut upiu) };
        let data = (dw7 & 0xFFFF != 0).then(|| {
            let mut e = [0u8; 16];
            unsafe { read(ucd + (dw7 >> 16) as u64 * 4, &mut e) };
            (u64::from_le_bytes(e[0..8].try_into().unwrap()), le32(&e, 12) as usize + 1)
        });
        let rsp = match upiu[0] {
            NOP_OUT => self.nop(),
            QUERY_REQ => self.query(&upiu),
            COMMAND => self.scsi(&upiu, dir, data),
            _ => None,
        };
        let Some(mut rsp) = rsp else { return };
        // The request's LUN and task tag come back in its response's header.
        (rsp[2], rsp[3]) = (upiu[2], upiu[3]);
        if let Some((_, at, v)) = self.lie.get().filter(|l| l.0 == rsp[0]) {
            rsp[at] = v;
        }
        unsafe {
            write(ucd + (dw6 >> 16) as u64 * 4, &rsp);
            let ocs = if upiu[0] == COMMAND { self.cfg.borrow().scsi_ocs } else { 0 };
            write(base + 8, &[ocs]);
        }
        let mut h = self.host.borrow_mut();
        h.regs[(UTRLDBR / 4) as usize] &= !1;
        h.regs[(IS / 4) as usize] |= IS_UTRCS;
    }
}
