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

//! An I2C request as the sink takes it: a write to 0x30 sets the segment,
//! a write to 0x50 the offset, a read from 0x50 gives bytes from there on,
//! an address-only request is acknowledged (a stop, unless the try's lie
//! is that its stop is never acknowledged).

use super::sink::{AuxSink, Eddc, Fault};

const ACK: u8 = 0x00;
const I2C_NACK: u8 = 0x40;

impl AuxSink {
    pub(super) fn i2c(&mut self, req: &[u8]) -> Vec<u8> {
        let (cmd, addr) =
            (req[0] >> 4, (req[0] as u32 & 0xF) << 16 | (req[1] as u32) << 8 | req[2] as u32);
        let n = req.get(3).map_or(0, |l| *l as usize + 1);
        let t = self.log.last_mut().unwrap();
        match (cmd & 0x9, addr, req.len()) {
            (_, _, 3) if t.fault == Fault::NoPhase => vec![I2C_NACK],
            (_, _, 3) => vec![ACK],
            (0, 0x30, 5) => {
                let ok = self.eddc != Eddc::Nak;
                t.segment = Some((req[4], ok || t.segment.is_some_and(|s| s.1)));
                if ok {
                    self.segment = req[4];
                }
                vec![if ok { ACK } else { I2C_NACK }]
            }
            (0, 0x50, 5) => {
                (t.offset, self.pos) = (Some(req[4]), 0);
                let (fault, pointed) = (t.fault, t.segment.is_some_and(|s| s.1));
                self.stream = self.bytes(req[4], fault, pointed);
                vec![ACK]
            }
            (1, 0x50, _) if t.offset.is_none() => {
                t.read_unaimed = true;
                vec![I2C_NACK]
            }
            (1, 0x50, _) => self.read(n),
            _ => vec![I2C_NACK],
        }
    }
}
