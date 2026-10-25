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

//! A transfer's AUX answer when it is not a clean one: no reply, a DEFER,
//! a NACK, an answer neither, or a reply with no command byte, drawn at
//! the sink's rates; and the try's stall, no reply once it is past its
//! byte. A request without middle-of-transaction under a stall still
//! answers: the stop of a stalled read.

use super::sink::{AuxSink, Fault};

impl AuxSink {
    /// Some(answer) when this transfer is not answered cleanly.
    pub(super) fn aux_fault(&mut self, req: &[u8]) -> Option<Option<Vec<u8>>> {
        let t = self.log.last().unwrap();
        let mot = req[0] & 0x40 != 0;
        if let (Fault::Stall(k), true) = (t.fault, mot) {
            if self.pos >= 4 * k && t.offset.is_some() {
                return Some(None);
            }
        }
        let r = self.rates;
        let roll = self.roll(100);
        let cuts = [r.timeout, r.defer, r.nack, r.undefined, r.empty];
        let mut at = 0;
        for (i, c) in cuts.into_iter().enumerate() {
            at += c;
            if roll < at {
                return Some(match i {
                    0 => None,
                    1 => Some(vec![[0x20, 0x80][self.roll(2) as usize]]),
                    2 => Some(vec![[0x10, 0x40][self.roll(2) as usize]]),
                    3 => Some(vec![0x30]),
                    _ => Some(Vec::new()),
                });
            }
        }
        None
    }
}
