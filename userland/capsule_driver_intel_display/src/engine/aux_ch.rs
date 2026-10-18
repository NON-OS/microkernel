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

//! One AUX transaction on a DDI's channel: wait until idle, load, send,
//! wait for Send/Busy to clear, then read the reply or the error flagged.

use super::access::{Clock, Expiry, Mmio, Until};
use super::aux_regs::{aux_ctl, aux_data, send_word, Ddi, DONE, RECEIVE_ERROR, SEND_BUSY};
use super::aux_regs::{SIZE_MASK, SIZE_SHIFT, TIMEOUT_ERROR};
use crate::dp::aux::{Aux, AuxError};

/// Longer than the hardware's own 1600 us reply timer plus the request.
const TRANSFER_US: u64 = 10_000;

pub struct AuxCh<'a> {
    pub mmio: &'a mut dyn Mmio,
    pub ddi: Ddi,
    pub clock: &'a dyn Clock,
}

impl AuxCh<'_> {
    fn idle(&mut self) -> Result<u32, AuxError> {
        let until = Until::new(self.clock, TRANSFER_US);
        loop {
            let ctl = self.mmio.read(aux_ctl(self.ddi));
            if ctl & SEND_BUSY == 0 {
                return Ok(ctl);
            }
            if until.expired() {
                return Err(AuxError::Timeout);
            }
        }
    }
}

impl Aux for AuxCh<'_> {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        if req.is_empty() || req.len() > 20 {
            return Err(AuxError::Short);
        }
        self.idle()?;
        for (i, chunk) in req.chunks(4).enumerate() {
            let mut w = [0u8; 4];
            w[..chunk.len()].copy_from_slice(chunk);
            self.mmio.write(aux_data(self.ddi, i as u32), u32::from_be_bytes(w));
        }
        self.mmio.write(aux_ctl(self.ddi), send_word(req.len()));
        let ctl = self.idle()?;
        // Clear what this transaction left before deciding on it.
        self.mmio.write(aux_ctl(self.ddi), ctl & (TIMEOUT_ERROR | RECEIVE_ERROR | DONE));
        if ctl & (TIMEOUT_ERROR | RECEIVE_ERROR) != 0 {
            return Err(AuxError::Timeout);
        }
        let len = (((ctl & SIZE_MASK) >> SIZE_SHIFT) as usize).min(20);
        for i in 0..len.div_ceil(4) {
            let w = self.mmio.read(aux_data(self.ddi, i as u32)).to_be_bytes();
            let n = (len - i * 4).min(4);
            reply[i * 4..i * 4 + n].copy_from_slice(&w[..n]);
        }
        Ok(len)
    }
}
