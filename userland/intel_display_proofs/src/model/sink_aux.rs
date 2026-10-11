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

//! The sink's AUX end: DPCD reads and writes, I2C to the EDID, DEFER and
//! NACK on demand.

use super::sink::Sink;
use crate::dp::aux::{Aux, AuxError};

pub struct AuxEnd(pub Sink);

const ACK: u8 = 0x00;
const NACK: u8 = 0x10;
const DEFER: u8 = 0x20;

impl Aux for AuxEnd {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        let mut s = self.0.borrow_mut();
        s.transfers += 1;
        if s.nack {
            reply[0] = NACK;
            return Ok(1);
        }
        if s.defers > 0 {
            s.defers -= 1;
            reply[0] = DEFER;
            return Ok(1);
        }
        let cmd = req[0] >> 4;
        let addr = (((req[0] & 0xF) as usize) << 16) | ((req[1] as usize) << 8) | req[2] as usize;
        let len = req.get(3).map(|&l| l as usize + 1).unwrap_or(0);
        reply[0] = ACK;
        match cmd {
            0x9 => {
                let st = s.status();
                for i in 0..len {
                    let a = addr + i;
                    reply[1 + i] =
                        if (0x202..0x208).contains(&a) { st[a - 0x202] } else { s.dpcd[a] };
                }
                Ok(1 + len)
            }
            0x8 => {
                s.dpcd[addr..addr + len].copy_from_slice(&req[4..4 + len]);
                Ok(1)
            }
            c if c & 0xB == 0x0 && req.len() == 5 => {
                s.i2c_offset = req[4] as usize;
                Ok(1)
            }
            c if c & 0xB == 0x1 => {
                for i in 0..len {
                    let o = s.i2c_offset;
                    reply[1 + i] = *s.edid.get(o).unwrap_or(&0xFF);
                    s.i2c_offset += 1;
                }
                Ok(1 + len)
            }
            _ => Ok(1),
        }
    }
}
