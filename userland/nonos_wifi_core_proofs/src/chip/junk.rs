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
//! What the lying chip gives between honest frames.

use super::liar::Liar;
use nonos_wifi_core::softmac::RxInfo;

impl Liar {
    pub(super) fn draw(&mut self) -> u64 {
        let s = &mut self.seed;
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    pub(super) fn junk(&mut self, out: &mut [u8]) -> RxInfo {
        let len = match self.draw() % 4 {
            0 => 0,
            1 => out.len() + 1 + (self.draw() % 5000) as usize,
            _ => (self.draw() % 300) as usize,
        };
        for b in out.iter_mut().take(len) {
            *b = self.draw() as u8;
        }
        let rssi_dbm = [i8::MIN, i8::MAX, 0][(self.draw() % 3) as usize];
        RxInfo { len, rssi_dbm, decrypted: self.draw().is_multiple_of(2) }
    }
}
