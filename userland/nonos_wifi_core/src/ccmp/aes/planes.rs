// NONOS Operating System (AGPL-3.0-or-later)

//! Sixteen bytes held as eight bit-planes, and GF(2^8) arithmetic on them:
//! plane `i` carries bit `i` of every byte, so one operation acts on all
//! sixteen bytes at once, with no table and no branch on their values.

/// Bit `i` of each of the sixteen bytes, byte `j` at bit `j` of plane `i`.
pub(super) type Planes = [u16; 8];

pub(super) fn split(s: &[u8; 16]) -> Planes {
    let mut p = [0u16; 8];
    for (i, plane) in p.iter_mut().enumerate() {
        for (j, b) in s.iter().enumerate() {
            *plane |= u16::from((b >> i) & 1) << j;
        }
    }
    p
}

pub(super) fn join(p: &Planes, s: &mut [u8; 16]) {
    for (j, b) in s.iter_mut().enumerate() {
        *b = 0;
        for (i, plane) in p.iter().enumerate() {
            *b |= (((plane >> j) & 1) as u8) << i;
        }
    }
}

/// Fold bits 8..15 of a product back below x^8 = x^4 + x^3 + x + 1.
fn reduce(c: &mut [u16; 15]) -> Planes {
    for k in (8..15).rev() {
        let hi = c[k];
        c[k - 4] ^= hi;
        c[k - 5] ^= hi;
        c[k - 7] ^= hi;
        c[k - 8] ^= hi;
    }
    let mut out = [0u16; 8];
    out.copy_from_slice(&c[..8]);
    out
}

/// The product in GF(2^8), each byte by its own.
pub(super) fn mul(a: &Planes, b: &Planes) -> Planes {
    let mut c = [0u16; 15];
    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            c[i + j] ^= ai & bj;
        }
    }
    reduce(&mut c)
}

/// The square: each bit moves to twice its place, then the fold.
pub(super) fn square(a: &Planes) -> Planes {
    let mut c = [0u16; 15];
    for (i, ai) in a.iter().enumerate() {
        c[2 * i] = *ai;
    }
    reduce(&mut c)
}

/// The inverse, x^254 (zero stays zero): x^3, x^7, x^15, then x^240 * x^14.
pub(super) fn invert(x: &Planes) -> Planes {
    let x3 = mul(&square(x), x);
    let x14 = square(&mul(&square(&x3), x));
    let x15 = mul(&x14, x);
    let x240 = square(&square(&square(&square(&x15))));
    mul(&x240, &x14)
}
