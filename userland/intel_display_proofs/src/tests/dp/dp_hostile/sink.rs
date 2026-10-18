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

//! The hostile sink's AUX end: each transfer answered as the profile says,
//! counted so a path that will not end fails the test, not hangs it.

use super::profile::Profile;
use super::rng::Rng;
use crate::dp::aux::{Aux, AuxError};
use crate::dp::sideband::refused;

pub struct Hostile {
    pub rng: Rng,
    pub p: Profile,
    pub dpcd: [u8; 16],
    pub transfers: u32,
    pub bound: u32,
    pub status_reads: u32,
    pub waited_us: u64,
    pub up: Option<(u32, u32)>,
    pub starts: u32,
}

impl Aux for Hostile {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        // A reply takes about 400 us; a timeout the transmitter's whole wait.
        (self.transfers, self.waited_us) = (self.transfers + 1, self.waited_us + 400);
        assert!(self.transfers <= self.bound, "{} transfers: a path did not end", self.transfers);
        let addr = (req[0] as u32 & 0xF) << 16 | (req[1] as u32) << 8 | req[2] as u32;
        assert!(!refused(addr, 1), "a sideband address {addr:#x} reached the bus");
        let p = self.p;
        let roll = self.rng.below(100);
        let cut = [p.timeout, p.defer, p.nack, p.undefined, p.empty];
        let mut at = 0;
        for (i, c) in cut.into_iter().enumerate() {
            at += c;
            if roll < at {
                return match i {
                    0 => Err(AuxError::Timeout).inspect_err(|_| self.waited_us += 19_600),
                    4 => Ok(0),
                    _ => {
                        reply[0] = [0, 0x20, 0x10, 0x30][i];
                        Ok(1)
                    }
                };
            }
        }
        reply[0] = 0;
        let asked = req.get(3).map_or(1, |l| *l as usize + 1);
        if req[0] >> 4 == 0x9 {
            self.answer(addr, &mut reply[1..1 + asked.min(16)]);
        }
        Ok(match self.rng.below(p.odd_len.max(1)) {
            0 if p.odd_len > 0 => 1 + self.rng.below(asked as u32 + 1) as usize,
            1 if p.odd_len > 0 => 255,
            _ => 1 + asked,
        })
    }
}
