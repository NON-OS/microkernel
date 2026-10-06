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

//! One byte into the decoder.

use super::state::{Step, Utf8};

impl Utf8 {
    pub fn push(&mut self, b: u8) -> Step {
        if self.need > 0 {
            if b & 0xC0 != 0x80 {
                self.reset();
                return Step::Invalid { again: true };
            }
            self.cp = (self.cp << 6) | (b & 0x3F) as u32;
            self.need -= 1;
            if self.need > 0 {
                return Step::Pending;
            }
            let (cp, min) = (self.cp, self.min);
            self.reset();
            if cp < min || (0xD800..=0xDFFF).contains(&cp) {
                return Step::Invalid { again: false };
            }
            return match char::from_u32(cp) {
                Some(c) => Step::Char(c),
                None => Step::Invalid { again: false },
            };
        }
        match b {
            0x00..=0x7F => Step::Char(b as char),
            0xC2..=0xDF => self.start(b & 0x1F, 1, 0x80),
            0xE0..=0xEF => self.start(b & 0x0F, 2, 0x800),
            0xF0..=0xF4 => self.start(b & 0x07, 3, 0x1_0000),
            _ => Step::Invalid { again: false },
        }
    }

    fn start(&mut self, bits: u8, need: u8, min: u32) -> Step {
        self.cp = bits as u32;
        self.need = need;
        self.min = min;
        Step::Pending
    }
}
