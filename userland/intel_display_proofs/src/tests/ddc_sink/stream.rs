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

//! The bytes a read is answered with: the EDID from the segment and index
//! the sink takes to be meant, or the DDC/CI responder's answer to the last
//! message written; then the try's lie in the bytes, if it has one. The
//! word address wraps inside a 256-byte segment, the model's choice; past
//! the EDID, bytes read 0xFF, as an unwritten EEPROM's would.

use super::sink::{Eddc, Fault, Sink};

impl Sink<'_> {
    pub(super) fn stream(&mut self, addr: u32, index: u32, len: usize, f: Fault) -> Vec<u8> {
        let mut s = match addr {
            0x50 => self.edid_at(index, len, f),
            0x37 => {
                let last = self.bus.ci_last.clone();
                self.ci.as_mut().map(|a| a(&last)).unwrap_or_default()
            }
            _ => Vec::new(),
        };
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

    fn edid_at(&self, index: u32, len: usize, f: Fault) -> Vec<u8> {
        let seg = match (self.eddc, self.bus.pointed) {
            (Eddc::Ignore, true) => 0,
            (Eddc::Off(k), true) => self.bus.segment.wrapping_add(k),
            _ => self.bus.segment,
        };
        let idx = match (f, self.ignore_index) {
            (Fault::Index(i), _) => i as usize,
            (_, true) => 0,
            _ => index as usize,
        };
        let at = |i: usize| seg as usize * 256 + (idx + i) % 256;
        (0..len + 8).map(|i| *self.edid.get(at(i)).unwrap_or(&0xFF)).collect()
    }
}
