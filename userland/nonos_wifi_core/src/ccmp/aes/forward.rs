// NONOS Operating System (AGPL-3.0-or-later)

//! The forward rounds' steps.

use super::tables::SBOX;

pub(super) fn add_round_key(s: &mut [u8; 16], k: &[u8]) {
    for (b, kb) in s.iter_mut().zip(k.iter()) {
        *b ^= *kb;
    }
}

pub(super) fn sub_bytes(s: &mut [u8; 16]) {
    for b in s.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

pub(super) fn shift_rows(s: &mut [u8; 16]) {
    // state is column-major: s[r + 4c]. row r rotates left by r.
    let t = *s;
    for r in 1..4 {
        for c in 0..4 {
            s[r + 4 * c] = t[r + 4 * ((c + r) % 4)];
        }
    }
}

pub(super) fn xtime(x: u8) -> u8 {
    (x << 1) ^ (0x1b & (((x >> 7) & 1).wrapping_neg()))
}

pub(super) fn mix_columns(s: &mut [u8; 16]) {
    for c in 0..4 {
        let col = [s[4 * c], s[4 * c + 1], s[4 * c + 2], s[4 * c + 3]];
        s[4 * c] = xtime(col[0]) ^ (xtime(col[1]) ^ col[1]) ^ col[2] ^ col[3];
        s[4 * c + 1] = col[0] ^ xtime(col[1]) ^ (xtime(col[2]) ^ col[2]) ^ col[3];
        s[4 * c + 2] = col[0] ^ col[1] ^ xtime(col[2]) ^ (xtime(col[3]) ^ col[3]);
        s[4 * c + 3] = (xtime(col[0]) ^ col[0]) ^ col[1] ^ col[2] ^ xtime(col[3]);
    }
}
