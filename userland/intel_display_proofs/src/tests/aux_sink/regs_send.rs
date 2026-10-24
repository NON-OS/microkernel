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

//! A request sent: the sink's answer put in the data registers with Done
//! and its size, or Done and the timeout flag; the clock moved on.

use super::regs::AuxRegs;
use super::sink::Fault;
use crate::engine::aux_regs::{DONE, SIZE_SHIFT, TIMEOUT_ERROR};

impl AuxRegs<'_> {
    pub(super) fn send(&mut self, len: usize) {
        let req: Vec<u8> = self.data.iter().flat_map(|w| w.to_be_bytes()).take(len).collect();
        let now = self.clock.now.get();
        let got = self.sink.transfer(&req, now);
        let slow = match self.sink.log.last().map(|t| t.fault) {
            Some(Fault::Slow(us)) => us,
            _ => 0,
        };
        let took = if got.is_some() { 400 } else { 1600 };
        self.clock.now.set(now + took + slow);
        self.ctl = match got {
            None => DONE | TIMEOUT_ERROR,
            Some(r) => {
                let n = r.len().min(20);
                for (i, c) in r[..n].chunks(4).enumerate() {
                    let mut w = [0u8; 4];
                    w[..c.len()].copy_from_slice(c);
                    self.data[i] = u32::from_be_bytes(w);
                }
                DONE | (n as u32) << SIZE_SHIFT
            }
        };
    }
}
