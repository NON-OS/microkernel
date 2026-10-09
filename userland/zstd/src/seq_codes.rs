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

//! Literal-length and match-length codes: a baseline and how many extra bits
//! follow it (RFC 8878 3.1.1.3.2.1.1). Past the direct codes each baseline is
//! the one before it plus the range its extra bits cover, so only the bit
//! counts are written down.

const LL_BITS: [u8; 20] = [1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
const ML_BITS: [u8; 21] = [1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

/// Codes below `direct` are their value plus `offset`; the rest walk `bits`.
fn code(code: u8, direct: u8, offset: u32, bits: &[u8]) -> Option<(u32, u8)> {
    if code < direct {
        return Some((u32::from(code) + offset, 0));
    }
    let index = usize::from(code - direct);
    let extra = *bits.get(index)?;
    let base = bits[..index].iter().fold(u32::from(direct) + offset, |b, &n| b + (1 << n));
    Some((base, extra))
}

pub fn literal_length(c: u8) -> Option<(u32, u8)> {
    code(c, 16, 0, &LL_BITS)
}

pub fn match_length(c: u8) -> Option<(u32, u8)> {
    code(c, 32, 3, &ML_BITS)
}
