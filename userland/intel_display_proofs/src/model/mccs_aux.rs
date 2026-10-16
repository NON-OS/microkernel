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

//! An AUX end with a DDC/CI monitor behind it, for the I2C-over-AUX
//! proofs: a write with middle-of-transaction set gathers the message, the
//! address-only write that ends it hands it to the monitor, and reads take
//! its reply; any address but 0x37 is NACKed. Every request is kept.

use super::mccs::Mccs;
use crate::dp::aux::{Aux, AuxError};

#[derive(Default)]
pub struct MccsAux {
    pub monitor: Mccs,
    pub msg: Vec<u8>,
    pub requests: Vec<Vec<u8>>,
}

impl Aux for MccsAux {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        self.requests.push(req.to_vec());
        let (cmd, addr) = (req[0] >> 4, ((req[1] as u32) << 8) | req[2] as u32);
        if cmd & 0x8 != 0 || addr != 0x37 {
            reply[0] = 0x40;
            return Ok(1);
        }
        reply[0] = 0;
        let (read, mot, len) =
            (cmd & 1 == 1, cmd & 4 != 0, req.get(3).map_or(0, |l| *l as usize + 1));
        match (read, mot) {
            (false, true) => self.msg.extend_from_slice(&req[4..4 + len]),
            (false, false) => self.monitor.take(&core::mem::take(&mut self.msg)),
            (true, true) => {
                for b in reply[1..1 + len].iter_mut() {
                    *b = self.monitor.next_byte();
                }
                return Ok(1 + len);
            }
            (true, false) => {}
        }
        Ok(1)
    }
}
