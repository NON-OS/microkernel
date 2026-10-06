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

/* Bits a fast lookup resolves at once; longer codes take the canonical walk. */
pub const FAST_BITS: u32 = 9;

/* Direct lookup for every code of at most FAST_BITS bits: indexed by the next
 * FAST_BITS stream bits (codes arrive most significant bit first, so each is
 * stored bit-reversed, once per value of the bits after it), an entry holds
 * symbol << 4 | length, 0 where no short code matches. An over-subscribed
 * length set has no table, so it decodes exactly as the canonical walk. */
pub fn fast_table(lengths: &[u8], counts: &[u16; 16]) -> [u16; 1 << FAST_BITS] {
    let mut table = [0u16; 1 << FAST_BITS];
    let mut left = 1i32;
    for &c in &counts[1..] {
        left = (left << 1) - c as i32;
        if left < 0 {
            return table;
        }
    }
    let mut next = [0u32; 16];
    let mut code = 0u32;
    for len in 1..16 {
        code = (code + counts[len - 1] as u32) << 1;
        next[len] = code;
    }
    for (sym, &l) in lengths.iter().enumerate() {
        let l = l as u32;
        if l == 0 || l > FAST_BITS {
            continue;
        }
        let rev = next[l as usize].reverse_bits() >> (32 - l);
        next[l as usize] += 1;
        let mut i = rev;
        while i < 1 << FAST_BITS {
            table[i as usize] = (sym as u16) << 4 | l as u16;
            i += 1 << l;
        }
    }
    table
}
