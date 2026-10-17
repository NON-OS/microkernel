// NONOS Operating System (AGPL-3.0-or-later)

//! The key schedule. Its S-box is computed (`sbox`), not tabled.

use super::forward::sub_bytes;
use crate::wipe::wipe;

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// The eleven round keys of `key`, FIPS-197 5.2.
pub(super) fn expand(key: &[u8; 16]) -> [u8; 176] {
    let mut w = [0u8; 176];
    w[..16].copy_from_slice(key);
    let mut i = 16;
    let mut rc = 0;
    while i < 176 {
        let mut t = [w[i - 4], w[i - 3], w[i - 2], w[i - 1]];
        if i % 16 == 0 {
            // RotWord, then SubWord through the computed S-box.
            let mut b = [0u8; 16];
            b[..4].copy_from_slice(&[t[1], t[2], t[3], t[0]]);
            sub_bytes(&mut b);
            t = [b[0] ^ RCON[rc], b[1], b[2], b[3]];
            wipe(&mut b);
            rc += 1;
        }
        for j in 0..4 {
            w[i + j] = w[i - 16 + j] ^ t[j];
        }
        i += 4;
    }
    w
}
