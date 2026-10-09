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

use super::huff::Table;
use super::huff_fill::short_codes;
use super::huff_sub::long_codes;
use super::types::End;

/// Builds `t` for the canonical code with lengths `lens` (0..=15, at most
/// 288 symbols); `meta` gives each symbol's kind, value and extra bits. An
/// over-subscribed code is corrupt; an incomplete one is kept, its missing
/// codes decoding as length 0.
pub fn build<const N: usize>(
    t: &mut Table<N>,
    lens: &[u8],
    meta: impl Fn(usize) -> u32,
) -> Result<(), End> {
    let mut count = [0u32; 16];
    for &l in lens {
        *count.get_mut(usize::from(l)).ok_or(End::Corrupt)? += 1;
    }
    count[0] = 0;
    let (mut left, mut next) = (1i64, [0u32; 16]);
    for len in 1..16 {
        left = (left << 1) - i64::from(count[len]);
        if left < 0 {
            return Err(End::Corrupt);
        }
        next[len] = (next[len - 1] + count[len - 1]) << 1;
    }
    /* Symbols in canonical order: by length, then by symbol. */
    let (mut sorted, mut at) = ([0u16; 288], [0usize; 17]);
    for l in 1..16 {
        at[l + 1] = at[l] + count[l] as usize;
    }
    for (sym, &l) in lens.iter().enumerate().filter(|(_, &l)| l != 0) {
        *sorted.get_mut(at[usize::from(l)]).ok_or(End::Corrupt)? = sym as u16;
        at[usize::from(l)] += 1;
    }
    let short = at[Table::<N>::BITS as usize];
    short_codes(t, lens, &sorted[..short], next, &meta);
    long_codes(t, lens, &sorted[short..at[15]], next, &meta);
    Ok(())
}
