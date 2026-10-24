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

//! A notice's line, written into a toast's worth of bytes.

pub(super) struct Line<'a> {
    pub(super) out: &'a mut [u8],
    pub(super) n: usize,
}

impl Line<'_> {
    pub(super) fn put(&mut self, bytes: &[u8]) {
        let take = bytes.len().min(self.out.len() - self.n);
        self.out[self.n..self.n + take].copy_from_slice(&bytes[..take]);
        self.n += take;
    }

    pub(super) fn number(&mut self, value: u8) {
        let digits = [b'0' + value / 100, b'0' + value / 10 % 10, b'0' + value % 10];
        let first = if value >= 100 {
            0
        } else if value >= 10 {
            1
        } else {
            2
        };
        self.put(&digits[first..]);
    }
}
