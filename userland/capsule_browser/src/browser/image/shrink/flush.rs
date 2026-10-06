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

use super::Shrink;

impl Shrink {
    /// Write the finished target row: alpha-weighted colour averages, with
    /// the mean alpha; a cell no source pixel reached stays transparent.
    pub(super) fn flush(&mut self) {
        let out = &mut self.px[self.row * self.dst.0..(self.row + 1) * self.dst.0];
        for (o, s) in out.iter_mut().zip(self.sums.iter_mut()) {
            if s[4] > 0 && s[3] > 0 {
                let ch = |v: u64| ((v + s[3] / 2) / s[3]).min(255) as u32;
                let a = ((s[3] + s[4] / 2) / s[4]).min(255) as u32;
                *o = (a << 24) | (ch(s[0]) << 16) | (ch(s[1]) << 8) | ch(s[2]);
            }
            *s = [0; 5];
        }
    }
}
