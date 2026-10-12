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

//! The digits typed for a passkey the device shows. A passkey is six
//! decimal digits, leading zeros among them (Vol 3, Part H, 2.3.5.3), so
//! six must be typed: fewer would send a different number than the device
//! shows, and pairing would fail on the peer's side.

pub const DIGITS: usize = 6;

#[derive(Default)]
pub struct Digits {
    typed: [u8; DIGITS],
    n: usize,
}

impl Digits {
    pub const fn new() -> Self {
        Self { typed: [0; DIGITS], n: 0 }
    }

    /// One more digit; anything else, or a seventh, is not taken.
    pub fn push(&mut self, c: u8) -> bool {
        if !c.is_ascii_digit() || self.n == DIGITS {
            return false;
        }
        self.typed[self.n] = c;
        self.n += 1;
        true
    }

    pub fn pop(&mut self) {
        self.n = self.n.saturating_sub(1);
        self.typed[self.n] = 0;
    }

    /// Wiped, not just shortened: the digits are the pairing's secret until
    /// it ends (Vol 3, Part H, 2.3.5.3).
    pub fn clear(&mut self) {
        self.typed = [0; DIGITS];
        self.n = 0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.typed[..self.n]
    }

    /// The passkey, once all six are typed.
    pub fn value(&self) -> Option<u32> {
        let all = self.n == DIGITS;
        all.then(|| self.typed.iter().fold(0u32, |v, d| v * 10 + u32::from(d - b'0')))
    }
}
