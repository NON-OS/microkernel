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

//! One byte of a release flipped, as tools/nonos-flip-byte flips it.

use super::fixture::Release;
use std::fs;

impl Release {
    /// The middle byte of `rel` inverted, as tools/nonos-flip-byte does.
    pub fn flip(&self, rel: &str) {
        let p = self.path().join(rel);
        let mut b = fs::read(&p).unwrap();
        let at = b.len() / 2;
        b[at] ^= 0xFF;
        fs::write(p, b).unwrap();
    }
}
