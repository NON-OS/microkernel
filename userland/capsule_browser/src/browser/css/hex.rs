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

/// `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa` (without the `#`) as ARGB; the
/// short forms double each nibble and a missing alpha is opaque.
pub fn parse_hex(h: &str) -> Option<u32> {
    let b = h.trim().as_bytes();
    let d = |i: usize| (b[i] as char).to_digit(16);
    let [r, g, bl, a] = match b.len() {
        3 => [d(0)? * 17, d(1)? * 17, d(2)? * 17, 255],
        4 => [d(0)? * 17, d(1)? * 17, d(2)? * 17, d(3)? * 17],
        6 => [d(0)? * 16 + d(1)?, d(2)? * 16 + d(3)?, d(4)? * 16 + d(5)?, 255],
        8 => {
            let a = d(6)? * 16 + d(7)?;
            [d(0)? * 16 + d(1)?, d(2)? * 16 + d(3)?, d(4)? * 16 + d(5)?, a]
        }
        _ => return None,
    };
    Some((a << 24) | (r << 16) | (g << 8) | bl)
}
