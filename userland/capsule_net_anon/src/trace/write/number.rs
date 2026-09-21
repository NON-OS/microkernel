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

//! Writing a decimal number into a line without a formatter.

use super::line::Line;

/// Widest decimal a u64 can reach, so the scratch buffer is never short.
const DIGITS_MAX: usize = 20;

impl Line {
    /// Append a space and a decimal number.
    ///
    pub fn num(&mut self, value: u64) -> &mut Self {
        self.text(b" ");
        let mut digits = [0u8; DIGITS_MAX];
        let mut count = 0usize;
        let mut left = value;
        loop {
            digits[count] = b'0' + (left % 10) as u8;
            left /= 10;
            count += 1;
            if left == 0 || count == DIGITS_MAX {
                break;
            }
        }
        for index in (0..count).rev() {
            self.text(&[digits[index]]);
        }
        self
    }
}
