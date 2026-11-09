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

//! The decimal digits of a count, which the widgets that show one share.

/// The decimal digits of a count, held without allocating.
pub(crate) struct Digits {
    buf: [u8; 10],
    start: usize,
}

impl Digits {
    /// The digits of `n`.
    pub(crate) fn of(mut n: u32) -> Self {
        let mut d = Digits { buf: [b'0'; 10], start: 10 };
        loop {
            d.start -= 1;
            d.buf[d.start] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                return d;
            }
        }
    }

    /// The digits as text.
    pub(crate) fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[self.start..]).unwrap_or("")
    }
}
