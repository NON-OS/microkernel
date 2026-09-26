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

//! A measurement, short enough to read off the screen.

/// The first six bytes.
pub fn short(m: &[u8; 32]) -> [u8; 12] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [0u8; 12];
    for i in 0..6 {
        out[i * 2] = HEX[(m[i] >> 4) as usize];
        out[i * 2 + 1] = HEX[(m[i] & 0xF) as usize];
    }
    out
}
