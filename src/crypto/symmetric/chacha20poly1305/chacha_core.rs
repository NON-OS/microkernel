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

//! The ChaCha20 state and its 20 rounds (RFC 8439 sections 2.1 to 2.3),
//! kept in sixteen locals so no round indexes an array.

const SIGMA: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

/// The input state for `key` and `nonce`, with the block counter at 0.
#[inline(always)]
pub(super) fn setup(key: &[u8; 32], nonce: &[u8; 12]) -> [u32; 16] {
    let mut s = [0u32; 16];
    s[..4].copy_from_slice(&SIGMA);
    for (w, b) in s[4..12].iter_mut().zip(key.as_chunks::<4>().0) {
        *w = u32::from_le_bytes(*b);
    }
    for (w, b) in s[13..].iter_mut().zip(nonce.as_chunks::<4>().0) {
        *w = u32::from_le_bytes(*b);
    }
    s
}

macro_rules! quarter_round {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        $a = $a.wrapping_add($b);
        $d = ($d ^ $a).rotate_left(16);
        $c = $c.wrapping_add($d);
        $b = ($b ^ $c).rotate_left(12);
        $a = $a.wrapping_add($b);
        $d = ($d ^ $a).rotate_left(8);
        $c = $c.wrapping_add($d);
        $b = ($b ^ $c).rotate_left(7);
    };
}

/// The key-stream words of the block `s` describes: the state after ten
/// double rounds, added word by word to `s`.
#[inline(always)]
pub(super) fn keystream(s: &[u32; 16]) -> [u32; 16] {
    let [mut x0, mut x1, mut x2, mut x3, mut x4, mut x5, mut x6, mut x7] =
        [s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]];
    let [mut x8, mut x9, mut x10, mut x11, mut x12, mut x13, mut x14, mut x15] =
        [s[8], s[9], s[10], s[11], s[12], s[13], s[14], s[15]];
    for _ in 0..10 {
        quarter_round!(x0, x4, x8, x12);
        quarter_round!(x1, x5, x9, x13);
        quarter_round!(x2, x6, x10, x14);
        quarter_round!(x3, x7, x11, x15);
        quarter_round!(x0, x5, x10, x15);
        quarter_round!(x1, x6, x11, x12);
        quarter_round!(x2, x7, x8, x13);
        quarter_round!(x3, x4, x9, x14);
    }
    let mut out = [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15];
    for (o, i) in out.iter_mut().zip(s) {
        *o = o.wrapping_add(*i);
    }
    out
}
