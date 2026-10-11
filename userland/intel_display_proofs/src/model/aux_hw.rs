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

//! The AUX channel hardware in the model: a write of the control register
//! with Send/Busy takes the message from the data registers, hands it to
//! the sink, and puts the reply back with Done and its size, or flags a
//! timeout when the sink gives none.

use super::gen9::Gen9;
use crate::engine::aux_regs::TIMEOUT_ERROR;
use crate::engine::aux_regs::{aux_ctl, aux_data, DONE, SEND_BUSY, SIZE_MASK, SIZE_SHIFT};

impl Gen9 {
    pub(super) fn aux_write(&mut self, off: u32, v: u32) {
        let Some((ddi, _)) = &self.aux else { return };
        let ddi = *ddi;
        if off != aux_ctl(ddi) {
            return;
        }
        if v & SEND_BUSY == 0 {
            // Clearing sticky bits: they go where ones were written.
            let old = self.get(off);
            self.regs.insert(off, old & !(v & 0x7E00_0000));
            return;
        }
        if self.aux_stuck {
            return;
        }
        let len = ((v & SIZE_MASK) >> SIZE_SHIFT) as usize;
        let mut req = Vec::new();
        for i in 0..len.div_ceil(4) {
            req.extend_from_slice(&self.get(aux_data(ddi, i as u32)).to_be_bytes());
        }
        req.truncate(len);
        let mut reply = [0u8; 20];
        let sink = &mut self.aux.as_mut().unwrap().1;
        let ctl = match sink.transfer(&req, &mut reply) {
            Ok(n) => {
                for i in 0..n.div_ceil(4) {
                    let w = u32::from_be_bytes(reply[i * 4..i * 4 + 4].try_into().unwrap());
                    self.regs.insert(aux_data(ddi, i as u32), w);
                }
                DONE | ((n as u32) << SIZE_SHIFT)
            }
            Err(_) => DONE | TIMEOUT_ERROR,
        };
        self.regs.insert(off, ctl);
    }
}
