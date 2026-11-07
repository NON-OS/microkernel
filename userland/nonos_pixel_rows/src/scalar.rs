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

//! The plain-Rust pixel rules every vector path must match byte for byte.

/// `upper` laid over `lower`, both premultiplied, each channel alpha
/// included: `u + round(l * (255 - ua) / 255)`, held at 255.
#[inline]
pub fn over(upper: u32, lower: u32) -> u32 {
    let keep = 255 - (upper >> 24);
    let mix = |shift: u32| {
        let (u, l) = ((upper >> shift) & 0xFF, (lower >> shift) & 0xFF);
        (u + div255(l * keep + 127)).min(255) << shift
    };
    mix(0) | mix(8) | mix(16) | mix(24)
}

/// `v / 255` for `v` up to 65152, with no divide: the vector paths use the
/// same shifts on 16-bit lanes.
#[inline]
pub fn div255(v: u32) -> u32 {
    (v + (v >> 8) + 1) >> 8
}

#[inline]
pub fn load(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}
