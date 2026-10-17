// NONOS Operating System (AGPL-3.0-or-later)

//! The inverse rounds' steps.

use super::sbox::inv_sub;

pub(super) fn inv_sub_bytes(s: &mut [u8; 16]) {
    inv_sub(s);
}

pub(super) fn inv_shift_rows(s: &mut [u8; 16]) {
    let t = *s;
    for r in 1..4 {
        for c in 0..4 {
            s[r + 4 * c] = t[r + 4 * ((c + 4 - r) % 4)];
        }
    }
}

// Multiply in GF(2^8) by masks, never by a branch on `a`, a byte of state.
pub(super) fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        p ^= a & (b & 1).wrapping_neg();
        a = (a << 1) ^ (0x1b & (a >> 7).wrapping_neg());
        b >>= 1;
    }
    p
}

pub(super) fn inv_mix_columns(s: &mut [u8; 16]) {
    for c in 0..4 {
        let col = [s[4 * c], s[4 * c + 1], s[4 * c + 2], s[4 * c + 3]];
        s[4 * c] = gmul(col[0], 14) ^ gmul(col[1], 11) ^ gmul(col[2], 13) ^ gmul(col[3], 9);
        s[4 * c + 1] = gmul(col[0], 9) ^ gmul(col[1], 14) ^ gmul(col[2], 11) ^ gmul(col[3], 13);
        s[4 * c + 2] = gmul(col[0], 13) ^ gmul(col[1], 9) ^ gmul(col[2], 14) ^ gmul(col[3], 11);
        s[4 * c + 3] = gmul(col[0], 11) ^ gmul(col[1], 13) ^ gmul(col[2], 9) ^ gmul(col[3], 14);
    }
}
