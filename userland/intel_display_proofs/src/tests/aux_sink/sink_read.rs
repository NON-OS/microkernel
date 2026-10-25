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

//! What a read gives: the bytes from the segment and offset the sink takes
//! to be meant, with the try's lie in them; NACK or no reply at the byte a
//! cut lands on; and now and then an ACK short of what was asked, or one
//! that runs past it with bytes the request did not ask for.

use super::sink::{AuxSink, Eddc, Fault};

impl AuxSink {
    pub(super) fn bytes(&self, offset: u8, f: Fault, pointed: bool) -> Vec<u8> {
        let seg = match (self.eddc, pointed) {
            (Eddc::Ignore, true) => 0,
            (Eddc::Off(k), true) => self.segment.wrapping_add(k),
            _ => self.segment,
        };
        let idx = match (f, self.ignore_index) {
            (Fault::Index(i), _) => i as usize,
            (_, true) => 0,
            _ => offset as usize,
        };
        let at = |i: usize| seg as usize * 256 + (idx + i) % 256;
        let mut s: Vec<u8> = (0..264).map(|i| *self.edid.get(at(i)).unwrap_or(&0xFF)).collect();
        match f {
            Fault::Drop(k) if k < s.len() => {
                s.remove(k);
            }
            Fault::Again(k) if 4 * k + 4 <= s.len() => {
                let w = s[4 * k..4 * k + 4].to_vec();
                s.splice(4 * k..4 * k, w);
            }
            _ => {}
        }
        s
    }

    pub(super) fn read(&mut self, n: usize) -> Vec<u8> {
        let fault = self.log.last().unwrap().fault;
        match fault {
            Fault::Nak(k) if self.pos >= 4 * k => return vec![0x40],
            _ => {}
        }
        let odd = self.rates.odd;
        let give = match self.roll(odd.max(1)) {
            0 if odd > 0 => 1 + self.roll(n as u32) as usize,
            1 if odd > 0 => 19,
            _ => n,
        };
        let take = give.min(n);
        let from = self.pos.min(self.stream.len());
        let data: Vec<u8> =
            (0..give).map(|i| *self.stream.get(from + i).unwrap_or(&0xFF)).collect();
        self.pos += take;
        self.log.last_mut().unwrap().got.extend_from_slice(&data[..take]);
        [&[0x00][..], &data].concat()
    }
}
