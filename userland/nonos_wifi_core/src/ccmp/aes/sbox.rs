// NONOS Operating System (AGPL-3.0-or-later)

//! The S-box and its inverse computed, not looked up: a table indexed by a
//! byte of key or state leaks that byte through the cache. FIPS-197 5.1.1:
//! the inverse in GF(2^8), then the affine map; and the reverse for the
//! inverse S-box.

use super::planes::{invert, join, split, Planes};

/// Every plane where `c` has a bit set, inverted.
fn add_constant(p: &mut Planes, c: u8) {
    for (i, plane) in p.iter_mut().enumerate() {
        *plane ^= 0u16.wrapping_sub(u16::from((c >> i) & 1));
    }
}

/// b ^ (b <<< 1) ^ (b <<< 2) ^ (b <<< 3) ^ (b <<< 4) ^ 0x63, per byte.
fn affine(b: &Planes) -> Planes {
    let mut s = [0u16; 8];
    for (i, si) in s.iter_mut().enumerate() {
        *si = b[i] ^ b[(i + 7) % 8] ^ b[(i + 6) % 8] ^ b[(i + 5) % 8] ^ b[(i + 4) % 8];
    }
    add_constant(&mut s, 0x63);
    s
}

/// (s <<< 1) ^ (s <<< 3) ^ (s <<< 6) ^ 0x05, per byte: the affine map undone.
fn inv_affine(s: &Planes) -> Planes {
    let mut b = [0u16; 8];
    for (i, bi) in b.iter_mut().enumerate() {
        *bi = s[(i + 7) % 8] ^ s[(i + 5) % 8] ^ s[(i + 2) % 8];
    }
    add_constant(&mut b, 0x05);
    b
}

/// The S-box on all sixteen bytes.
pub(super) fn sub(s: &mut [u8; 16]) {
    let p = affine(&invert(&split(s)));
    join(&p, s);
}

/// The inverse S-box on all sixteen bytes.
pub(super) fn inv_sub(s: &mut [u8; 16]) {
    let p = invert(&inv_affine(&split(s)));
    join(&p, s);
}
